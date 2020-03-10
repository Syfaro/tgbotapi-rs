# tgbotapi

An async Rust library for working with the Telegram Bot API.

**This project is not yet complete but provides most of the core experience.**

It makes no assumptions about how you are using the API and only provides types and wrappers around the Bot API.

It defines a common trait, `TelegramRequest` that all requests implement. This trait supports dynamic endpoints, arbitrary values (as long as they can be serialized into JSON with serde), and file uploads (including multiple files).

## Examples

```rust
use tgbotapi::{Telegram, requests::{ChatID, GetUpdates, SendMessage}};

// Create a new Telegram instance
let telegram = Telegram::new("api_token".into());

// Create a request to get updates with long-polling enabled
let mut get_updates = GetUpdates {
    timeout: Some(30),
    ..Default::default()
};

// Loop forever getting new updates
loop {
    // Ask Telegram for new updates
    for update in telegram.make_request(&get_updates).await? {
        // Increment the offset to tell Telegram we processed the update
        get_updates.offset = Some(update.update_id + 1);

        // Ignore all non-message updates
        let message = match update.message {
            Some(message) => message,
            _ => continue,
        };

        // Create a request to send a message
        let send_message = SendMessage {
            chat_id: message.chat_id(),
            text: "Hello, world!".into(),
            ..Default::default()
        };

        // Send the message
        telegram.make_request(&send_message).await?;
    }
}
```

For higher-performance bots you should use webhooks.

```rust
use tgbotapi::Update;

// Get the body of the request from your web handler
let body = get_request_body!();
let update: Update = serde_json::from_slice(&body)?;

// Now we can do something with the update
```
