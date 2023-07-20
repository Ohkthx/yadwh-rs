//! Represents the messages sent and received from the Discord API.
//!
//! `message` contains Message and MessageResponse.
//! `Message` is what is used to send data to the Discord API.
//! `MessageResponse` is received from the API on message creation, edit, and obtaining.

use crate::embed::Embed;
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
#[derive(Serialize, Debug)]
pub struct Message {
    /// Overrides the default username of the webhook.
    pub username: String,
    /// The message contents (up to 2000 characters)
    pub content: String,
    /// Embedded `rich` content, an array of up to 10 embeds.
    pub embeds: Vec<Embed>,
}

impl Message {
    /// Create a new message to be sent to the API.
    ///
    /// # Arguments
    ///
    /// * `username` - Overrides the webhook's username.
    /// * `content` - Message to be sent (up to 2000 characters)
    pub fn new(username: &str, content: &str) -> Self {
        Self {
            username: username.to_string(),
            content: content.to_string(),
            embeds: vec![],
        }
    }

    /// Creates a new embed to be added to the list of embeds to be sent.
    pub fn embed<Func>(&mut self, func: Func) -> &mut Self
    where
        Func: Fn(&mut Embed) -> &mut Embed,
    {
        let mut embed = Embed::new();
        func(&mut embed);
        self.embeds.push(embed);

        self
    }
}
