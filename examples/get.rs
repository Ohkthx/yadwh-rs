//! # Get Message Example
//!
//! This example demonstrates how to obtain a discord webhook message that originated from a
//! specific Webhook ID, Token, and Message ID. The arguments provided should be provided exactly
//! in that order.
//!
//! ## Example
//!
//! cargo run --example get --features examples -- 00001111 aaaabbbb 22223333
//! where:
//!     Webhook ID: 00001111
//!     Token:      aaaabbbb
//!     Message ID: 22223333
use std::{env, process};
use yadwh::Webhook;

#[tokio::main]
async fn main() {
    // Verify enough arguments were passed.
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("error:  not enough arguments supplied.");
        println!("usage:  get [webhook_id] [token] [message_id]");
        process::exit(-1);
    }

    // Parse the arguments.
    let webhook_id: String = args[1].to_string();
    let token: String = args[2].to_string();
    let message_id: String = args[3].to_string();

    // Get the message.
    println!("Obtaining message {}.", message_id);
    let webhook = Webhook::new(&webhook_id, &token);
    match webhook.get(&message_id).await {
        Ok(resp) => println!("\nMessage obtained:\n{:#?}", resp),
        Err(error) => println!("Error while obtaining: {}", error),
    }
}
