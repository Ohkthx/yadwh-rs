//! # Create Message Example
//!
//! This example demonstrates how to create a discord webhook message that originated from a
//! specific Webhook ID and Token. The arguments provided should be provided exactly
//! in that order.
//!
//! ## Example
//!
//! cargo run --example create_message --features examples -- 00001111 aaaabbbb
//! where:
//!     Webhook ID: 00001111
//!     Token:      aaaabbbb

use std::{env, process};
use yadwh::message::MessageBuilder;
use yadwh::webhook::WebhookApi;

#[tokio::main]
async fn main() -> Result<(), yadwh::WebhookError> {
    // Verify enough arguments were passed.
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("error: not enough arguments supplied.");
        println!("usage: create_message [webhook_id] [token]");
        process::exit(-1);
    }

    // Parse the arguments.
    let webhook_id: String = args[1].to_string();
    let token: String = args[2].to_string();

    // Message to be sent.
    let message = MessageBuilder::new()
        .username("Webhook Example")?
        .content("Content portion of the message.")?
        .embed(|embed| {
            embed
                .color("#cba6f7")
                .author("Author Here", None, None, None)
                .title("Title Here")
                .description("Description Here\n```rust\nprintln!(\"Hello World!\");```")
                .field("Field1", "Value1", None)
                .field("Inline Field1", "Value1", Some(true))
                .field("Inline Field2", "Value2", Some(true))
                .field("Inline Field3", "Value3", Some(true))
                .footer("Footer Here", None, None)
        });

    // Create the message.
    println!("Creating message.");
    let webhook = WebhookApi::new(&webhook_id, &token);
    match webhook.message.create(&message, None).await {
        Ok(resp) => println!("\nMessage created:\n{:#?}", resp),
        Err(error) => println!("Error while creating: {}", error),
    }

    Ok(())
}
