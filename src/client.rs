//! Webhook Client for interacting directly with the Discord API.
//!
//! `webhook` bundles up the required authentication parameters and creates a HTTP client that is
//! used to interact with the Discord API. All authentication for each request is handled for the user.

use hyper::body::Buf;
use hyper::client::{Client as HyperClient, HttpConnector};
use hyper::{Body, Method, Request, StatusCode};
use hyper_tls::HttpsConnector;
use std::fmt;

/// Base URI for the Webhook API.
pub(crate) const ROOT_URI: &str = "https://discord.com/api/v10/webhooks";

/// Used to return either objects or errors.
pub type Result<T> = std::result::Result<T, WebhookError>;

/// Enum for handling the expected errors for processing webhook messages.
pub enum WebhookError {
    /// Non-200 status obtained from the API.
    BadStatus(String),
    /// 204 Response received from the API.
    NoContent,
    /// Unknown error, details provided.
    Unknown(String),
    /// Unable to parse an object received from the API.
    BadParse(String),
    /// Content or Embed character count is too large.
    TooBig(String, usize, usize),
}

impl fmt::Display for WebhookError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WebhookError::BadStatus(value) => write!(f, "bad status: {}", value),
            WebhookError::BadParse(value) => write!(f, "bad parse: {}", value),
            WebhookError::Unknown(value) => write!(f, "unknown: {}", value),
            WebhookError::NoContent => f.write_str("no content."),
            WebhookError::TooBig(value, size, max) => write!(
                f,
                "{} exceeded max character count, {} of {}",
                value, size, max
            ),
        }
    }
}

/// Collection of Limits enforced by the Discord API.
///
/// ## References / Documentation
///
/// <https://discord.com/developers/docs/resources/channel#embed-object-embed-limits>
/// <https://discord.com/developers/docs/resources/channel#create-message>
pub struct Limit;
impl Limit {
    /// Maximum amount of embeds allowed on a single message.
    pub const EMBEDS: usize = 10;
    /// Maximum amount of fields on a single embed.
    pub const FIELDS: usize = 25;

    /// Maximum length of a username override for a message.
    pub const USERNAME: usize = 80;
    /// Maximum length of content for a message.
    pub const CONTENT: usize = 2000;

    /// Maximum length of the author name on an embed.
    pub const AUTHOR_NAME: usize = 256;
    /// Maximum length of the title on an embed.
    pub const TITLE: usize = 256;
    /// Maximum length of the description on an embed.
    pub const DESCRIPTION: usize = 4096;
    /// Maximum length of field name on an embed.
    pub const FIELD_NAME: usize = 256;
    /// Maximum length of field value on an embed.
    pub const FIELD_VALUE: usize = 1024;
    /// Maximum length of footer text on an embed.
    pub const FOOTER_TEXT: usize = 2048;
    /// Maximum total characters for an embed.
    pub const EMBED_TOTAL: usize = 6000;
}

/// Webhook is a client that is responsible for making requests to the Discord API.
/// Requires a Webhook ID and Token. You can find these requirements in the URL provided for the
/// webhook.
///
/// Example:
#[allow(rustdoc::bare_urls)]
/// URL Supplied: https://discord.com/api/webhooks/__111122223333__/**AAAABBBBCCCC**
///
/// Webhook ID: __111122223333__
///
/// Webhook Token: **AAAABBBBCCCC**
#[derive(Debug, Clone)]
pub(crate) struct Client {
    /// ID of the Webhook.
    pub id: String,
    /// Token for the Webhook.
    pub token: String,
    /// HTTP client used to send requests to the API.
    client: HyperClient<HttpsConnector<HttpConnector>>,
}

impl Client {
    /// Creates a new Webhook client used to send requests.
    ///
    /// # Arguments
    ///
    /// * `webhook_id` - ID of the Webhook.
    /// * `webhook_token` - Token of the Webhook.
    pub fn new(webhook_id: &str, webhook_token: &str) -> Self {
        let connector = HttpsConnector::new();
        Self {
            id: webhook_id.to_string(),
            token: webhook_token.to_string(),
            client: HyperClient::builder().build::<_, Body>(connector),
        }
    }

    /// Used to create the base endpoint.
    fn url(&self) -> String {
        format!("{}/{}/{}", ROOT_URI, self.id, self.token)
    }

    /// Sends requests to the Discord API.
    ///
    /// # Arguments
    ///
    /// * `method` - Method to perform, valid options are: Method::GET, Method::POST,
    /// Method::DELETE, and Method::PATCH.
    /// * `endpoint` - Target endpoint to access.
    /// * `body` - HTTP Body to send to the API (used for POST and PATCH.)
    pub async fn send(&self, method: Method, endpoint: &str, body: Body) -> Result<String> {
        let url = format!("{}{}", self.url(), endpoint);

        // Build the request for the Method.
        let req = Request::builder()
            .method(method)
            .uri(url)
            .header("Content-Type", "application/json")
            .body(body);

        // Send the request, parse the response.
        match self.client.request(req.unwrap()).await {
            Ok(value) => match value.status() {
                StatusCode::OK => {
                    // Convert the HTTP body stream to a &[u8]
                    let body = match hyper::body::to_bytes(value).await {
                        Ok(data) => data,
                        Err(_) => {
                            return Err(WebhookError::Unknown(
                                "unable to convert http body".to_string(),
                            ))
                        }
                    };

                    // Convert to JSON string to be parsed by calling function and return.
                    match std::str::from_utf8(body.chunk()) {
                        Ok(data) => Ok(data.to_owned()),
                        Err(_) => Err(WebhookError::Unknown(
                            "unable to convert to json".to_string(),
                        )),
                    }
                }

                // Bad status code received, print the code.
                StatusCode::NO_CONTENT => Err(WebhookError::NoContent),
                _ => {
                    let code = format!("Status Code: {}", value.status().as_u16());
                    Err(WebhookError::BadStatus(format!("{}", code)))
                }
            },

            // Non-status code error while processing response.
            Err(_) => Err(WebhookError::Unknown("request to API".to_string())),
        }
    }
}
