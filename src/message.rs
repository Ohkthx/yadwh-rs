//! Contains `MessageAPI`, which is used to create, get, edit, and delete messages.
//!
//! This is used by proxy in `WebhookAPI` to manage messages.

use crate::client::{Client, Limit, Result, WebhookError};
use crate::embed::Embed;
use hyper::{Body, Method};
use serde::{Deserialize, Serialize};

/// Message received from the Discord API after message creation, edit, and obtaining.
///
/// ## References / Documentation
///
/// <https://discord.com/developers/docs/resources/channel#message-object>
#[derive(Deserialize, Debug)]
pub struct Message {
    /// ID of the message.
    pub id: String,
    /// ID of the channel the message was sent in.
    pub channel_id: String,
    /// Contents of the message.
    pub content: String,
    /// When this message was sent.
    pub timestamp: String,
    /// When this message was last edited.
    pub edited_timestamp: Option<String>,
    /// Whether this was a TTS (Text-to-Speech) message.
    pub tts: bool,
    /// Whether this message mentions everyone.
    pub mention_everyone: bool,
    /// Any embedded content.
    pub embeds: Vec<Embed>,
    /// Whether this message is pinned.
    pub pinned: bool,
    /// This is the webhook's ID.
    pub webhook_id: String,
    /// Type of message.
    pub r#type: u8,
}

/// Used to build a message to be sent to the API. At least one of content or embeds must be
/// included to be a valid message.
///
/// ## References / Documentation
///
/// <https://discord.com/developers/docs/resources/webhook#execute-webhook-jsonform-params>
#[derive(Serialize, Debug, Default)]
pub struct MessageBuilder {
    /// Overrides the default username of the webhook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// The message contents (up to 2000 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// True if this is a TTS (Text-to-Speech) message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<bool>,
    /// Embedded `rich` content, an array of up to 10 embeds.
    pub embeds: Vec<Embed>,
}

impl MessageBuilder {
    /// Create a new message to be sent to the API. This requires either an embed or content to be
    /// set.
    pub fn new() -> Self {
        Self {
            embeds: vec![],
            ..Default::default()
        }
    }

    /// Converts a `Message` into a `MessageBuilder` to be further modified.
    ///
    /// # Arguments
    ///
    /// * `message` - Message used to create the builder.
    pub fn from(message: &Message) -> Self {
        let mut builder = Self::new();
        builder.content(&message.content).ok();
        builder.embeds = message.embeds.clone();
        builder
    }

    /// Validates the entire embed does not exceed the maxmium lengths. Maximium length is availabe
    /// in `Limit::EMBED_TOTAL`. Returns the total size for all embeds within the message.
    pub fn validate(&self) -> Result<usize> {
        let too_big = |name: &str, size: usize, max: usize| -> WebhookError {
            WebhookError::TooBig(name.to_string(), size, max)
        };

        // Check if the username is too large.
        match &self.username {
            Some(value) => match value.len() {
                0..=Limit::USERNAME => (),
                _ => return Err(too_big("username", value.len(), Limit::USERNAME)),
            },
            None => (),
        };

        // Check if the content is too large.
        match &self.content {
            Some(value) => match value.len() {
                0..=Limit::CONTENT => (),
                _ => return Err(too_big("content", value.len(), Limit::CONTENT)),
            },
            None => (),
        };

        // Check the total size of all embeds attached.
        let mut total: usize = 0;
        for embed in self.embeds.iter() {
            total += match embed.validate() {
                Ok(value) => value,
                Err(error) => return Err(error),
            }
        }

        // Verify the total is less than embed max.
        match total {
            0..=Limit::EMBED_TOTAL => Ok(total),
            _ => Err(too_big("embed", total, Limit::EMBED_TOTAL)),
        }
    }

    /// Overrides the username for the message. This will throw a `WebhookError::TooBig` if the
    /// username exceeds the maximum length (currently 80 characters, see: `Limit::USERNAME`).
    ///
    /// # Arguments
    ///
    /// * `username` - Username to be display for the message, maximum length is `Limit::USERNAME`
    pub fn username(&mut self, username: &str) -> Result<()> {
        // Assign, but will not send if it is an error.
        self.username = Some(username.to_string());

        // Throw an error if it is too long.
        if username.len() > Limit::USERNAME {
            return Err(WebhookError::TooBig(
                "username".to_string(),
                username.len(),
                Limit::USERNAME,
            ));
        };

        Ok(())
    }

    /// Adds content to the message. This will throw a `WebhookError::TooBig` if the content
    /// exceeds the maximum length (currently 2000 characters).
    ///
    /// # Arguments
    ///
    /// * `content` - String of content to be sent, maximum length is `Limit::CONTENT`
    pub fn content(&mut self, content: &str) -> Result<()> {
        // Assign, but will not send if it is an error.
        self.content = Some(content.to_string());

        // Throw an error if it is too long.
        if content.len() > Limit::CONTENT {
            return Err(WebhookError::TooBig(
                "content".to_string(),
                content.len(),
                Limit::CONTENT,
            ));
        };

        Ok(())
    }

    /// Explictly sets TTS (Text-to-Speech) to either be `true` or `false`
    ///
    /// # Arguments
    ///
    /// * `tts` - `true` or `false` to `enable` or `disable` TTS (Text-to-Speech.)
    pub fn tts(&mut self, tts: bool) {
        self.tts = Some(tts);
    }

    /// Creates a new embed to be added to the list of embeds to be sent. If you attempt to add
    /// more then 10 embeds, it will fail and only keep the first 10.
    pub fn embed<Func>(&mut self, func: Func) -> &mut Self
    where
        Func: Fn(&mut Embed) -> &mut Embed,
    {
        if self.embeds.len() < Limit::EMBEDS {
            let mut embed = Embed::new();
            func(&mut embed);
            self.embeds.push(embed);
        }

        self
    }
}

/// `MessageAPI` is used to negotiate `Message` related functions with the Discord API. This allows
/// the user to **Create**, **Get**, **Edit**, and **Delete** messages sent by the webhook. This is
/// accessed by proxy in `WebhookAPI`.
pub struct MessageAPI {
    /// HTTP client used to send requests to the API.
    client: Client,
}

impl MessageAPI {
    /// Creates a new instance of the MessageAPI by cloning a HTTP client provided by the calling
    /// WebhookAPI.
    ///
    /// # Arguments
    ///
    /// * `client` - HTTP Client used to authenticate, send, and receive information.
    pub(crate) fn new(client: &Client) -> Self {
        Self {
            client: client.clone(),
        }
    }

    /// Creates a new message via the webhook with the supplied message. The `thread_id` is
    /// required if message is to be created inside of a Forum Channel Thread.
    ///
    /// # Arguments
    ///
    /// * `message` - Message to send to the API.
    /// * `thread_id` - Required if the webhook is posting in a Forum Channel's Thread, otherwise ignore.
    ///
    /// ## References / Documentation
    ///
    /// <https://discord.com/developers/docs/resources/webhook#execute-webhook>
    pub async fn create(
        &self,
        message: &MessageBuilder,
        thread_id: Option<&str>,
    ) -> Result<Message> {
        // Validate the message.
        match message.validate() {
            Ok(_) => (),
            Err(error) => return Err(error),
        };

        // '?wait=true' tells the API to return the message with the newly created ID.
        let mut url = "?wait=true".to_string();
        url = match thread_id {
            Some(value) => format!("{}&thread_id={}", url, value),
            None => url,
        };

        let body = Body::from(serde_json::to_string(message).unwrap());

        // Send a POST request to create the new webhook message.
        match self.client.send(Method::POST, &url, body).await {
            Ok(value) => match serde_json::from_str(&value) {
                Ok(resp) => Ok(resp),
                Err(_) => Err(WebhookError::BadParse("create response".to_string())),
            },
            Err(error) => Err(error),
        }
    }

    /// Obtains an existing message sent by the webhook. This will error if it no longer exists.
    ///
    /// # Arguments
    ///
    /// * `id` - ID of the message to obtain from the API.
    ///
    /// ## References / Documentation
    ///
    /// <https://discord.com/developers/docs/resources/webhook#get-webhook-message>
    pub async fn get(&self, id: &str) -> Result<Message> {
        // Path to the actual message being accessed.
        let url = format!("/messages/{}", id);
        let body = Body::from("");

        // Send a GET request to obtain an existing webhook message.
        match self.client.send(Method::GET, &url, body).await {
            Ok(value) => match serde_json::from_str(&value) {
                Ok(resp) => Ok(resp),
                Err(_) => Err(WebhookError::BadParse("get response".to_string())),
            },
            Err(error) => Err(error),
        }
    }

    /// Edits an existing message sent by the webhook. This will error if it no longer exists.
    ///
    /// # Arguments
    ///
    /// * `id` - ID of the message to edit.
    /// * `message` - Message used to replace the already existing message.
    ///
    /// ## References / Documentation
    ///
    /// <https://discord.com/developers/docs/resources/webhook#edit-webhook-message>
    pub async fn edit(&self, id: &str, message: &MessageBuilder) -> Result<Message> {
        // Validate the message.
        match message.validate() {
            Ok(_) => (),
            Err(error) => return Err(error),
        }

        // Path to the actual message being modified.
        let url = format!("/messages/{}", id);
        let body = Body::from(serde_json::to_string(message).unwrap());

        // Send a PATCH request to change an existing webhook message.
        match self.client.send(Method::PATCH, &url, body).await {
            Ok(value) => match serde_json::from_str(&value) {
                Ok(resp) => Ok(resp),
                Err(_) => Err(WebhookError::BadParse("edit response".to_string())),
            },
            Err(error) => Err(error),
        }
    }

    /// Deletes an existing message sent by the webhook. Any 'Ok' response indicates success.
    ///
    /// # Arguments
    ///
    /// * `id` - ID of the message to delete.
    ///
    /// ## References / Documentation
    ///
    /// <https://discord.com/developers/docs/resources/webhook#delete-webhook-message>
    pub async fn delete(&self, id: &str) -> Result<()> {
        // Path to the actual message being modified.
        let url = format!("/messages/{}", id);
        let body = Body::from("");

        // Send a DELETE request to remove an existing webhook message.
        match self.client.send(Method::DELETE, &url, body).await {
            Ok(_) => Ok(()),
            Err(error) => match error {
                WebhookError::NoContent => Ok(()),
                _ => Err(error),
            },
        }
    }
}
