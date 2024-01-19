//! # Delete Message Example
//!
//! This example demonstrates how to delete a discord webhook message that originated from a
//! specific Webhook ID, Token, and Message ID. The arguments provided should be provided exactly
//! in that order.
//!
//! ## Example
//!
//! cargo run --example delete_message --features examples -- 00001111 aaaabbbb 22223333
//! where:
//!     Webhook ID: 00001111
//!     Token:      aaaabbbb
//!     Message ID: 22223333

use std::{env, process};
use yadwh::webhook::WebhookApi;

#[tokio::main]
async fn main() {
    // Verify enough arguments were passed.
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("error:  not enough arguments supplied.");
        println!("usage:  delete_message [webhook_id] [token] [message_id]");
        process::exit(-1);
    }

    // Parse the arguments.
    let webhook_id: String = args[1].to_string();
    let token: String = args[2].to_string();
    let message_id: String = args[3].to_string();

    // Delete the message.
    println!("Deleting message {}.", message_id);
    let webhook = WebhookApi::new(&webhook_id, &token);
    match webhook.message.delete(&message_id).await {
        Ok(_) => println!("Deleted message {}", message_id),
        Err(error) => println!("Error while deleting: {}", error),
    }
}
