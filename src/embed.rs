//! Embed Object that is optionally sent in messages.
//!
//! `embed` contains the Embed struct used to be sent with messages to the Discord API. Up to 10
//! embeds can be sent per message.

use crate::client::{Limit, WebhookError};
use serde::{Deserialize, Serialize};

/// Author information for the embed.
///
/// ## References / Documentation
///
/// <https://discord.com/developers/docs/resources/channel#embed-object-embed-author-structure>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbedAuthor {
    /// Name of the author.
    pub name: String,
    /// URL of the author (only supports http(s)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// URL to the icon for the author (only supports http(s) and attachments).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// Proxy URL to the icon for the author.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_icon_url: Option<String>,
}

/// Fields information for the embed.
///
/// ## References / Documentation
///
/// <https://discord.com/developers/docs/resources/channel#embed-object-embed-field-structure>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbedField {
    /// Name of the field.
    pub name: String,
    /// Value of the field.
    pub value: String,
    /// Whether or not this field should display inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline: Option<bool>,
}

/// Footer information for the embed.
///
/// ## References / Documentation
///
/// <https://discord.com/developers/docs/resources/channel#embed-object-embed-footer-structure>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbedFooter {
    /// Footer text.
    pub text: String,
    /// URL of the footer icon (only supports http(s) and attachments)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// A proxied URL of the footer icon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_icon_url: Option<String>,
}

/// Image, Video, or Thumbnail information for the embed.
///
/// ## References / Documentation
///
/// <https://discord.com/developers/docs/resources/channel#embed-object-embed-thumbnail-structure>
/// <https://discord.com/developers/docs/resources/channel#embed-object-embed-video-structure>
/// <https://discord.com/developers/docs/resources/channel#embed-object-embed-image-structure>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbedMedia {
    /// Source URL of thumbnail (only supports http(s) and attachments)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// A proxied URL of the media.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<String>,
    /// Height of the media.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    /// Width of the media.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
}

/// Provider information for the embed.
///
/// ## References / Documentation
///
/// <https://discord.com/developers/docs/resources/channel#embed-object-embed-provider-structure>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbedProvider {
    /// Name of the provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// URL of the provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Embed is an optional object that can be sent with a message to Discord. Up to 10 embeds can
/// exist for any single message.
///
/// ## References / Documentation
///
/// <https://discord.com/developers/docs/resources/channel#embed-object>
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Embed {
    /// Author information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<EmbedAuthor>,
    /// Title of the embed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Description of the embed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// URL of the embed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Timestamp of the embed content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// color code of the embed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<u32>,
    /// Fields information.
    pub fields: Vec<EmbedField>,
    /// Footer information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<EmbedFooter>,
    /// Image information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<EmbedMedia>,
    /// Thumbnail information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<EmbedMedia>,
    /// Video information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<EmbedMedia>,
    /// Provider information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<EmbedProvider>,
}

impl Embed {
    /// Creates a new instance of an Embed.
    pub fn new() -> Self {
        Self {
            fields: vec![],
            ..Default::default()
        }
    }

    /// Validates the Embed does not exceed the maxmium lengths. Returns to the total amount of
    /// characters within the embed.
    pub fn validate(&self) -> Result<usize, WebhookError> {
        let too_big = |name: &str, size: usize, max: usize| -> WebhookError {
            WebhookError::TooBig(name.to_string(), size, max)
        };

        let mut total: usize = 0;

        // Check if the author is too large.
        let author = match &self.author {
            Some(value) => value.name.len(),
            None => 0,
        };
        total += match author {
            0..=Limit::AUTHOR_NAME => author,
            _ => return Err(too_big("author", author, Limit::AUTHOR_NAME)),
        };

        // Check if the title is too large.
        let title = match &self.title {
            Some(value) => value.len(),
            None => 0,
        };
        total += match title {
            0..=Limit::TITLE => title,
            _ => return Err(too_big("title", title, Limit::TITLE)),
        };

        // Check if the description is too large.
        let desc = match &self.description {
            Some(value) => value.len(),
            None => 0,
        };
        total += match desc {
            0..=Limit::DESCRIPTION => desc,
            _ => return Err(too_big("description", desc, Limit::DESCRIPTION)),
        };

        // Check if the footer is too large.
        let footer = match &self.footer {
            Some(value) => value.text.len(),
            None => 0,
        };
        total += match footer {
            0..=Limit::FOOTER_TEXT => footer,
            _ => return Err(too_big("footer", footer, Limit::FOOTER_TEXT)),
        };

        // Check all of the fields.
        for field in self.fields.iter() {
            // Check if the name is too large.
            let name = field.name.len();
            total += match name {
                0..=Limit::FIELD_NAME => name,
                _ => return Err(too_big("field name", name, Limit::FIELD_NAME)),
            };

            // Check if the value is too large.
            let value = field.value.len();
            total += match value {
                0..=Limit::FIELD_VALUE => value,
                _ => return Err(too_big("field value", value, Limit::FIELD_VALUE)),
            };
        }

        // Verify the total is less than embed max.
        match total {
            0..=Limit::EMBED_TOTAL => Ok(total),
            _ => Err(too_big("embed", total, Limit::EMBED_TOTAL)),
        }
    }

    /// Sets the title for the Embed.
    ///
    /// # Arguments
    ///
    /// * `title` - Title of the embed.
    pub fn title(&mut self, title: &str) -> &mut Self {
        self.title = Some(title.to_string());
        self
    }

    /// Sets the description for the Embed.
    ///
    /// # Arguments
    ///
    /// * `description` - Description of the embed.
    pub fn description(&mut self, description: &str) -> &mut Self {
        self.description = Some(description.to_string());
        self
    }

    /// Sets the url for the Embed.
    ///
    /// # Arguments
    ///
    /// * `url` - URL to assign to the embed.
    pub fn url(&mut self, url: &str) -> &mut Self {
        self.url = Some(url.to_string());
        self
    }

    /// Sets the timestamp for the Embed.
    ///
    /// # Arguments
    ///
    /// * `timestamp` - Timestamp to assign to the embed.
    pub fn timestamp(&mut self, timestamp: &str) -> &mut Self {
        self.timestamp = Some(timestamp.to_string());
        self
    }

    /// Sets the color (in hex, such as AA11BB or #AA11BB) for the Embed.
    ///
    /// # Arguments
    ///
    /// * `color` - Color to assign to the embed.
    pub fn color(&mut self, color: &str) -> &mut Self {
        // Remove the '#' prefix if it exists.
        let color_hex = match color.is_empty() {
            true => return self,
            false => match color.strip_prefix('#') {
                Some(value) => value,
                None => color,
            },
        };

        // Convert the HEX color to u32.
        let color_u32: u32 = match u32::from_str_radix(&color_hex, 16) {
            Ok(value) => value,
            Err(_) => return self,
        };
        self.color = Some(color_u32);

        self
    }

    /// Sets the footer for the Embed.
    ///
    /// # Arguments
    ///
    /// * `text` - Text for the footer.
    /// * `icon_url` - URL for the icon.
    /// * `proxy_icon_url` - Proxy URL for the icon to assign to the embed.
    pub fn footer(
        &mut self,
        text: &str,
        icon_url: Option<String>,
        proxy_icon_url: Option<String>,
    ) -> &mut Self {
        self.footer = Some(EmbedFooter {
            text: text.to_string(),
            icon_url,
            proxy_icon_url,
        });

        self
    }

    /// Sets the image information for the Embed.
    ///
    /// # Arguments
    ///
    /// * `url` - URL for the image.
    /// * `proxy_url` - Proxy URL for the image.
    /// * `height` - Height of the image.
    /// * `width` - Width of the image.
    pub fn image(
        &mut self,
        url: Option<String>,
        proxy_url: Option<String>,
        height: Option<u32>,
        width: Option<u32>,
    ) -> &mut Self {
        self.image = Some(EmbedMedia {
            url,
            proxy_url,
            height,
            width,
        });

        self
    }

    /// Sets the thumbnail information for the Embed.
    ///
    /// # Arguments
    ///
    /// * `url` - URL for the thumbnail.
    /// * `proxy_url` - Proxy URL for the thumbnail.
    /// * `height` - Height of the thumbnail.
    /// * `width` - Width of the thumbnail.
    pub fn thumbnail(
        &mut self,
        url: Option<String>,
        proxy_url: Option<String>,
        height: Option<u32>,
        width: Option<u32>,
    ) -> &mut Self {
        self.thumbnail = Some(EmbedMedia {
            url,
            proxy_url,
            height,
            width,
        });

        self
    }

    /// Sets the video information for the Embed.
    ///
    /// # Arguments
    ///
    /// * `url` - URL for the video.
    /// * `proxy_url` - Proxy URL for the video.
    /// * `height` - Height of the video.
    /// * `width` - Width of the video.
    pub fn video(
        &mut self,
        url: Option<String>,
        proxy_url: Option<String>,
        height: Option<u32>,
        width: Option<u32>,
    ) -> &mut Self {
        self.video = Some(EmbedMedia {
            url,
            proxy_url,
            height,
            width,
        });

        self
    }

    /// Sets the provider information for the embed.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the provider.
    /// * `url` - URL for the provider.
    pub fn provider(&mut self, name: Option<String>, url: Option<String>) -> &mut Self {
        self.provider = Some(EmbedProvider { name, url });

        self
    }

    /// Sets the author information for the embed.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the author.
    /// * `url` - URL of the author.
    /// * `icon_url` - URL for the icon of the author.
    /// * `proxy_icon_url` - Proxy URL for the icon of the author.
    pub fn author(
        &mut self,
        name: &str,
        url: Option<String>,
        icon_url: Option<String>,
        proxy_icon_url: Option<String>,
    ) -> &mut Self {
        self.author = Some(EmbedAuthor {
            name: name.to_string(),
            url,
            icon_url,
            proxy_icon_url,
        });

        self
    }

    /// Creates a field for the embed.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the field.
    /// * `value` - Value of the field.
    /// * `inline` - Whether or not the field is an inline field.
    pub fn field(&mut self, name: &str, value: &str, inline: Option<bool>) -> &mut Self {
        let field = EmbedField {
            name: name.to_string(),
            value: value.to_string(),
            inline,
        };

        self.fields.push(field);

        self
    }
}
