//! # Delete Webhook Example
//!
//! This example demonstrates how to delete a discord webhook that originated from a
//! specific Webhook ID and Token. The arguments provided should be provided exactly
//! in that order.
//!
//! ## Example
//!
//! cargo run --example delete_webhook --features examples -- 00001111 aaaabbbb
//! where:
//!     Webhook ID: 00001111
//!     Token:      aaaabbbb

use std::{env, process};
use yadwh::webhook::WebhookApi;

#[tokio::main]
async fn main() {
    // Verify enough arguments were passed.
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("error:  not enough arguments supplied.");
        println!("usage:  delete_webhook [webhook_id] [token]");
        process::exit(-1);
    }

    // Parse the arguments.
    let webhook_id: String = args[1].to_string();
    let token: String = args[2].to_string();

    // Delete the webhook.
    println!("Deleting webhook {}.", webhook_id);
    let webhook = WebhookApi::new(&webhook_id, &token);
    match webhook.delete().await {
        Ok(_) => println!("Deleted webhook {}.", webhook_id),
        Err(error) => println!("Error while deleting: {}", error),
    }
}
