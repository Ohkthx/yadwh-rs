<p align="center">
    <a href="https://crates.io/crates/yadwh" title="crates.io version.">
        <img src="https://img.shields.io/crates/v/yadwh?style=for-the-badge&logoColor=89b4fa&labelColor=11111b&color=89b4fa"
            alt="crates.io version"></a>
    <a href="https://crates.io/crates/yadwh" title="crates.io download counter.">
        <img src="https://img.shields.io/crates/d/yadwh?style=for-the-badge&logoColor=89dceb&labelColor=11111b&color=89dceb"
            alt="crates.io downloads"></a>
    <a href="https://github.com/ohkthx/yadwh-rs" title="Size of the repo!">
        <img src="https://img.shields.io/github/repo-size/Ohkthx/yadwh-rs?style=for-the-badge&logoColor=a6e3a1&labelColor=11111b&color=a6e3a1"
</p>

# Yet Another Discord Webhook

The objective of this crate is to grant asynchronous access to the **Discord Webhook** API. Beyond managing webhooks, this crate also allows for users to edit, obtain, and delete existing messages created by the webhook. There are several other crates that exist with similar functionality, however, I felt they were with missing features or not updated.

Contributions are encouraged! The API reference can be seen at [Discord Webhook API](https://discord.com/developers/docs/resources/webhook). If you wish to add this to your project, either use `cargo add yadwh` or add the following line to your dependencies section in **Cargo.toml**:

```toml
[dependencies]
yadwh = { git = "https://github.com/ohkthx/yadwh-rs" }
```

## Features
- Asynchronous.
- Easy-to-use Webhook Client.
- Get, Edit, and Delete Webhooks.
- Create, Get, Edit, and Delete Webhook messages.

## Documentation

Most of the documentation can be accessed by clicking the following link: [docs.rs](https://docs.rs/yadwh/latest/yadwh/). That documentation is automatically generated and also accessible from [crates.io](https://crates.io/crates/yadwh).

### Covered API requests

Client: `use yadwh::webhook::WebhookAPI` or `use yadwh::WebhookAPI`, create with `WebhookAPI::new()`

**Webhook API**:
- **Get Webhook**: `WebhookAPI.get`
- **Modify Webhook**: `WebhookAPI.modify`
- **Delete Webhook**: `WebhookAPI.delete`
- **MessageAPI**: `WebhookAPI.message`
  - **Create Message**: `WebhookAPI.message.create`
  - **Get Message**: `WebhookAPI.message.get`
  - **Edit Message**: `WebhookAPI.message.edit`
  - **Delete Message**: `WebhookAPI.message.delete`

### TODO

- Support files / attachments.

## Examples

Check above in the **Covered API requests** section for possibly covered examples. **NOTE FOR BELOW**: `AAAABBBB` is the Webhook ID, `11112222` is the Webhook Token, and `CCCCDDDD` is the Message ID for some requests. 

### WebhookAPI

- **Get Webhook**: [get_webhook.rs](https://github.com/Ohkthx/yadwh-rs/tree/main/examples/get_webhook.rs)
  - Try with: `cargo run --example get_webhook --features examples -- AAAABBBB 11112222 CCCCDDDD`
- **Modify Webhook**: [modify_webhook.rs](https://github.com/Ohkthx/yadwh-rs/tree/main/examples/modify_webhook.rs)
  - Try with: `cargo run --example modify_webhook --features examples -- AAAABBBB 11112222 CCCCDDDD`
- **Delete Webhook**: [delete_webhook.rs](https://github.com/Ohkthx/yadwh-rs/tree/main/examples/delete_webhook.rs)
  - Try with: `cargo run --example delete_webhook --features examples -- AAAABBBB 11112222 CCCCDDDD`

### MessageAPI

- **Create Message**: [create_message.rs](https://github.com/Ohkthx/yadwh-rs/tree/main/examples/create_message.rs)
  - Try with: `cargo run --example create_message --features examples -- AAAABBBB 11112222`
- **Create Thread Message (Forum Channel)**: [create_thread_message.rs](https://github.com/Ohkthx/yadwh-rs/tree/main/examples/create_thread_message.rs)
  - Try with: `cargo run --example create_thread_message --features examples -- AAAABBBB 11112222 CCCCDDDD`
- **Get Message**: [get_message.rs](https://github.com/Ohkthx/yadwh-rs/tree/main/examples/get_message.rs)
  - Try with: `cargo run --example get_message --features examples -- AAAABBBB 11112222 CCCCDDDD`
- **Edit Message**: [edit_message.rs](https://github.com/Ohkthx/yadwh-rs/tree/main/examples/edit_message.rs)
  - Try with: `cargo run --example edit_message --features examples -- AAAABBBB 11112222 CCCCDDDD`
- **Delete Message**: [delete_message.rs](https://github.com/Ohkthx/yadwh-rs/tree/main/examples/delete_message.rs)
  - Try with: `cargo run --example delete_message --features examples -- AAAABBBB 11112222 CCCCDDDD`

## Tips Appreciated!

Wallet addresses are provided below.
```
Ethereum (ETH): 0x7d75f6a9c021fcc70691fec73368198823fb0f60
Bitcoin (BTC):  bc1q75w3cgutug8qdxw3jlmqnkjlv9alt3jr7ftha0
Binance (BNB):  0x7d75f6a9c021fcc70691fec73368198823fb0f60
```
