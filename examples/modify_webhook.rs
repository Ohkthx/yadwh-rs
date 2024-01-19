//! # Modify Webhook Example
//!
//! This example demonstrates how to edit a discord webhook that originated from a
//! specific Webhook ID and Token. The arguments provided should be provided exactly
//! in that order.
//!
//! ## Example
//!
//! cargo run --example modify_webhook --features examples -- 00001111 aaaabbbb
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
        println!("usage:  modify_webhook [webhook_id] [token]");
        process::exit(-1);
    }

    // Parse the arguments.
    let webhook_id: String = args[1].to_string();
    let token: String = args[2].to_string();

    // Get the webhook.
    println!("Obtaining webhook {}.", webhook_id);
    let webhook = WebhookApi::new(&webhook_id, &token);
    let mut old = match webhook.get().await {
        Ok(resp) => {
            println!("Webhook obtained.");
            resp
        }
        Err(error) => {
            println!("Error while obtaining: {}", error);
            process::exit(-1);
        }
    };

    // Change the username.
    old.name = "NEW USERNAME".to_string();

    // Edit the webhook.
    println!("Editing webhook {}.", webhook_id);
    match webhook.modify(&old).await {
        Ok(resp) => println!("\nWebhook edited:\n{:#?}", resp),
        Err(error) => println!("Error while editing: {}", error),
    }
}
