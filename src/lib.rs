//! # Yet Another Discord Webhook
//!
//! The objective of this crate is to grant asynchronous access to the **Discord Webhook** API. Beyond creating webhook messages, this crate also allows for users to edit, obtain, and delete existing messages created by the webhook. There are several other crates that exist with similar functionality, however, I felt they were with missing features or not updated.
//!
//! Contributions are encouraged! The API reference can be seen at [Discord Webhook API](https://discord.com/developers/docs/resources/webhook). If you wish to add this to your project, either use `cargo add yadwh` or add the following line to your dependencies section in **Cargo.toml**:

#![cfg_attr(all(test, feature = "full"), deny(unreachable_pub))]
#![cfg_attr(all(test, feature = "full"), deny(warnings))]

pub mod embed;
pub mod message;
pub mod webhook;

/// Shortcut to webhook::Webhook
pub use crate::webhook::Webhook;
