//! # Edit Message Example
//!
//! This example demonstrates how to edit a discord webhook message that originated from a
//! specific Webhook ID, Token, and Message ID. The arguments provided should be provided exactly
//! in that order.
//!
//! ## Example
//!
//! cargo run --example edit --features examples -- 00001111 aaaabbbb 22223333
//! where:
//!     Webhook ID: 00001111
//!     Token:      aaaabbbb
//!     Message ID: 22223333
use std::{env, process};
use yadwh::{message::Message, Webhook};

#[tokio::main]
async fn main() {
    // Verify enough arguments were passed.
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("error:  not enough arguments supplied.");
        println!("usage:  edit [webhook_id] [token] [message_id]");
        process::exit(-1);
    }

    // Parse the arguments.
    let webhook_id: String = args[1].to_string();
    let token: String = args[2].to_string();
    let message_id: String = args[3].to_string();

    // Message to be sent.
    let mut message = Message::new("Webhook Example", "Below are updated attached embeds.");
    message.embed(|embed| {
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
    let webhook = Webhook::new(&webhook_id, &token);
    match webhook.edit(&message_id, &message).await {
        Ok(resp) => println!("\nMessage edited:\n{:#?}", resp),
        Err(error) => println!("Error while editing: {}", error),
    }
}
