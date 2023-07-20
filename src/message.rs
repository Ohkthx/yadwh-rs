//! Represents the messages sent and received from the Discord API.
//!
//! `message` contains Message and MessageResponse.
//! `Message` is what is used to send data to the Discord API.
//! `MessageResponse` is received from the API on message creation, edit, and obtaining.

use crate::embed::Embed;
use crate::webhook::{Limit, WebhookError};
use serde::{Deserialize, Serialize};

/// Message received from the Discord API after message creation, edit, and obtaining.
///
/// ## References / Documentation
///
/// <https://discord.com/developers/docs/resources/channel#message-object>
#[derive(Deserialize, Debug)]
pub struct MessageResponse {
    /// ID of the message.
    pub id: String,
    /// ID of the channel the message was sent in.
    pub channel_id: String,
    /// Contents of the message.
    pub content: String,
    /// When this message was sent.
    pub timestamp: String,
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

/// Represents a message sent to the API.
///
/// ## References / Documentation
///
/// <https://discord.com/developers/docs/resources/webhook#execute-webhook-jsonform-params>
#[derive(Serialize, Debug, Default)]
pub struct Message {
    /// Overrides the default username of the webhook.
    pub username: Option<String>,
    /// The message contents (up to 2000 characters)
    pub content: Option<String>,
    /// Embedded `rich` content, an array of up to 10 embeds.
    pub embeds: Vec<Embed>,
}

impl Message {
    /// Create a new message to be sent to the API. This requires either an embed or content to be
    /// set.
    pub fn new() -> Self {
        Self {
            embeds: vec![],
            ..Default::default()
        }
    }

    /// Validates the enter embed does not exceed the maxmium lengths. Returns the total size for
    /// all embeds within the message.
    pub fn validate(&self) -> Result<usize, WebhookError> {
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
    pub fn username(&mut self, username: &str) -> Result<(), WebhookError> {
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
    pub fn content(&mut self, content: &str) -> Result<(), WebhookError> {
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
