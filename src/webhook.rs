//! WebhookApi Client for interacting directly with the Discord API.
//!
//! `webhook` bundles up the required authentication parameters and creates a HTTP client that is
//! used to interact with the Discord API. All authentication for each request is handled for the user.

use crate::client::{Client, Result, WebhookError};
use crate::message::MessageApi;
use hyper::{Body, Method};
use serde::{Deserialize, Serialize};

/// Webhook object that contains all of the information regarding a Discord Webhook.
///
/// ## References / Documentation
///
/// <https://discord.com/developers/docs/resources/webhook#webhook-object-webhook-structure>
#[derive(Serialize, Deserialize, Debug)]
pub struct Webhook {
    /// ID of the webhook.
    pub id: String,
    /// Type of the webhook.
    pub r#type: u8,
    /// Guild ID this webhook is for, if any.
    pub guild_id: String,
    /// Channel ID this webhook is for, if any.
    pub channel_id: String,
    /// Default name of the webhook.
    pub name: String,
    /// Default user avatar hash of the webhook.
    pub avatar: Option<String>,
    /// Secure token of the webhook (returned for Incoming webhooks.)
    pub token: String,
    /// URL used for executing the webhook (returned by the webhooks OAuth2 flow.)
    pub url: String,
}

/// WebhookApi is a client that is responsible for making requests to the Discord API.
/// Requires a webhook ID and Token. You can find these requirements in the URL provided for the
/// webhook.
///
/// Example:
#[allow(rustdoc::bare_urls)]
/// URL Supplied: https://discord.com/api/webhooks/__111122223333__/**AAAABBBBCCCC**
/// * Webhook ID: __111122223333__
/// * Webhook Token: **AAAABBBBCCCC**
pub struct WebhookApi {
    /// HTTP client used to send requests to the API.
    client: Client,
    /// HTTP client used to send requests to the API.
    pub message: MessageApi,
}

impl WebhookApi {
    /// Creates a new webhook client used to send requests.
    ///
    /// # Arguments
    ///
    /// * `webhook_id` - ID of the webhook.
    /// * `webhook_token` - Token of the webhook.
    pub fn new(webhook_id: &str, webhook_token: &str) -> Self {
        let client: Client = Client::new(webhook_id, webhook_token);
        let message: MessageApi = MessageApi::new(&client);
        Self { client, message }
    }

    /// Parses a Discord webhook URL and creates a new `WebhookApi` client.
    ///
    /// # Arguments
    ///
    /// * `url` - The full URL of the webhook.
    ///
    /// # Returns
    ///
    /// A `WebhookApi` instance if the URL is valid, otherwise returns an error.
    pub fn from_url(url: &str) -> Result<Self> {
        let parts: Vec<&str> = url.split('/').collect();
        if parts.len() < 7 {
            return Err(WebhookError::BadParse("webhook url".to_string()));
        }

        let webhook_id = parts[parts.len() - 2];
        let webhook_token = parts[parts.len() - 1];

        Ok(Self::new(webhook_id, webhook_token))
    }

    /// Obtains an existing webhook. This will error if it no longer exists.
    ///
    /// ## References / Documentation
    ///
    /// <https://discord.com/developers/docs/resources/webhook#get-webhook-with-token>
    pub async fn get(&self) -> Result<Webhook> {
        // Send a GET request to obtain an existing webhook.
        match self.client.send(Method::GET, "", Body::from("")).await {
            Ok(value) => match serde_json::from_str(&value) {
                Ok(resp) => Ok(resp),
                Err(_) => Err(WebhookError::BadParse("get response".to_string())),
            },
            Err(error) => Err(error),
        }
    }

    /// Modifies an existing webhook. This will error if it no longer exists.
    ///
    /// # Arguments
    ///
    /// * `webhook` - Webhook used to replace the already existing webhook.
    ///
    /// ## References / Documentation
    ///
    /// <https://discord.com/developers/docs/resources/webhook#modify-webhook-with-token>
    pub async fn modify(&self, webhook: &Webhook) -> Result<Webhook> {
        // Webhook converted to an HTTP Body.
        let body = Body::from(serde_json::to_string(webhook).unwrap());

        // Send a PATCH request to change an existing webhook message.
        match self.client.send(Method::PATCH, "", body).await {
            Ok(value) => match serde_json::from_str(&value) {
                Ok(resp) => Ok(resp),
                Err(_) => Err(WebhookError::BadParse("edit response".to_string())),
            },
            Err(error) => Err(error),
        }
    }

    /// Deletes the existing webhook. Any 'Ok' response indicates success.
    ///
    /// ## References / Documentation
    ///
    /// <https://discord.com/developers/docs/resources/webhook#delete-webhook-with-token>
    pub async fn delete(&self) -> Result<()> {
        // Send a DELETE request to remove an existing webhook.
        match self.client.send(Method::DELETE, "", Body::from("")).await {
            Ok(_) => Ok(()),
            Err(error) => match error {
                WebhookError::NoContent => Ok(()),
                _ => Err(error),
            },
        }
    }
}
