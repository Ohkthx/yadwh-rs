//! # Edit Message Example
//!
//! This example demonstrates how to edit a discord webhook message that originated from a
//! specific Webhook ID, Token, and Message ID. The arguments provided should be provided exactly
//! in that order.
//!
//! ## Example
//!
//! cargo run --example edit_message --features examples -- 00001111 aaaabbbb 22223333
//! where:
//!     Webhook ID: 00001111
//!     Token:      aaaabbbb
//!     Message ID: 22223333
use std::{env, process};
use yadwh::message::MessageBuilder;
use yadwh::webhook::WebhookAPI;

#[tokio::main]
async fn main() {
    // Verify enough arguments were passed.
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("error:  not enough arguments supplied.");
        println!("usage:  edit_message [webhook_id] [token] [message_id]");
        process::exit(-1);
    }

    // Parse the arguments.
    let webhook_id: String = args[1].to_string();
    let token: String = args[2].to_string();
    let message_id: String = args[3].to_string();

    // Get the original message.
    println!("Obtaining message {}.", message_id);
    let webhook = WebhookAPI::new(&webhook_id, &token);
    let message = match webhook.message.get(&message_id).await {
        Ok(resp) => {
            println!("Message obtained.");
            resp
        }
        Err(error) => {
            println!("Error while obtaining: {}", error);
            process::exit(-1);
        }
    };

    // Create a builder from the message to make changes.
    let mut builder = MessageBuilder::from(&message);

    // Override the username. Ignoring error check for exceeding length.
    builder.username("Webhook Example").ok();

    // Set the content, check to make sure the length is within limits.
    match builder.content("New content portion of the message.") {
        Ok(_) => (),
        Err(error) => println!("{}", error),
    };

    // Create an embed for the message.
    builder.embeds = vec![];
    builder.embed(|embed| {
        embed
            .color("#cba6f7")
            .author("Author Changed Here", None, None, None)
            .title("Title Changed Here")
            .description("Description Changed Here\n```rust\nprintln!(\"Hello World!\");```")
            .field("Field1 Changed", "Value1", None)
            .field("Inline Field1", "Changed Value1", Some(true))
            .field("Inline Field2", "Value2", Some(true))
            .field("Inline Field3", "Value3", Some(true))
            .footer("Footer Here", None, None)
    });

    // Edit the message.
    println!("Editing message {}.", message_id);
    let webhook = WebhookAPI::new(&webhook_id, &token);
    match webhook.message.edit(&message_id, &builder).await {
        Ok(resp) => println!("\nMessage edited:\n{:#?}", resp),
        Err(error) => println!("Error while editing: {}", error),
    }
}
