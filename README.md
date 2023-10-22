# Nylas Rust SDK

[![Rust](https://img.shields.io/badge/Rust-1.50%2B-blue.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/License-MIT-brightgreen.svg)](LICENSE)

💌 A Rust client library for the Nylas email and calendar API.

## Overview

This Nylas Rust SDK is a library that provides Rust developers with easy access to the Nylas email and calendar API. Nylas is a powerful platform for building email, calendar, and contact functionality into applications.

🌟 **Key Features:**

- Retrieve email messages, threads, and threads.
- Access event and calendar information.
- Interact with folders, labels, and tags.
- Manage accounts and account details.
- Utilize powerful search and filter capabilities.
- And much more!

🚀 This client library empowers you to integrate the Nylas API seamlessly into your Rust applications.

## Installation

To include the Nylas Rust SDK in your project, add it as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
nylas = "0.0.7"
tokio = "1.33.0"
```

You can find the latest version on [Crates.io](https://crates.io/crates/nylas).

## Usage

🔐 **Authentication**: Start by setting up authentication and obtaining an access token.

```rust
use nylas::client::Nylas;

#[tokio::main]
async fn main() {
    let client_id = "YOUR_CLIENT_ID";
    let client_secret = "YOUR_CLIENT_SECRET";

    // Initialize the Nylas client with client ID and secret
    let mut nylas = Nylas::new(client_id, client_secret, None).await.unwrap();

    // Define authentication parameters
    let redirect_uri = "http://localhost:3000";
    let login_hint = Some("user@example.com");
    let state = Some("unique_identifier");
    let scopes = Some("email,calendar,contacts");

    // Generate an authentication URL
    match nylas.authentication_url(redirect_uri, login_hint, state, scopes) {
        Ok(auth_url) => println!("Authentication URL: {}", auth_url),
        Err(error) => eprintln!("Error: {}", error),
    }

    // Set the access token exchanged
    let access_token = "YOUR_ACCESS_TOKEN";
    nylas = Nylas::new(client_id, client_secret, Some(access_token))
        .await
        .unwrap();
}
```

📧 **Retrieve All Messages**: Fetch all messages from the Nylas API.

```rust
// ...

// Call the all method to retrieve all messages
let messages = nylas.messages().all().await;

match messages {
    Ok(messages) => {
        for message in messages {
            // Process each message
            println!("Message To: {:?}", message.to);
        }
    }
    Err(err) => {
        // Handle the error
        eprintln!("Error: {}", err);
    }
}
```

🔍 **Search for Messages**: Search for messages based on a query.

```rust
// ...

// Call the `search` method to search for messages
let query = "user@example.com";
let limit = Some(1);
let offset = Some(0);
let result = nylas.messages().search(query, limit, offset).await;

match result {
    Ok(messages) => {
        for message in messages {
            // Process each searched message
            println!("Searched Message: {:?}", message);
        }
    }
    Err(err) => {
        // Handle the error
        eprintln!("Error: {}", err);
    }
}
```

🔍 **Filter Messages**: Filter messages based on specified criteria with an optional view parameter.

```rust
// ...

// Define filter parameters as a HashMap
let mut filter = HashMap::new();
filter.insert("to", "user@example.com");

// Call the `where_` method with filter and view parameters
let messages = nylas.messages().where_(Some(filter), Some(View::Expanded)).await;

match messages {
    Ok(messages) => {
        for message in messages {
            // Process each message
            println!("Message ID: {}", message.id);
            // Access other fields as necessary
        }
    }
    Err(err) => {
        // Handle the error
        eprintln!("Error: {}", err);
    }
}
```

📨 **Retrieve the First Message**: Get the most recent message from the Nylas API.

```rust
// ...

let message_result = nylas.messages().first().await;

match message_result {
    Ok(Some(message)) => {
        // Process the first message
        println!("First Message: {:?}", message);
    }
    Ok(None) => {
        // Handle the case when there are no messages
        println!("No messages found.");
    }
    Err(err) => {
        // Handle the error
        eprintln!("Error: {}", err);
    }
}
```

🔍 **Retrieve a Specific Message by ID**: Get a specific message by its ID.

```rust
// ...

let message_id = "YOUR_MESSAGE_ID";
let message_result = nylas.messages().get(message_id, None).await;

match message_result {
    Ok(Some(message)) => {
        // Process the retrieved message
        println!("Message ID: {:?}", message.id);
        // In expanded view mode, you can access message.headers, etc.
    }
    Ok(None) => {
        // Handle the case when the message is not found
        println!("Message not found.");
    }
    Err(err) => {
        // Handle the error
        eprintln!("Error: {}", err);
    }
}
```

TODO: Explore the [examples](examples) folder for more usage scenarios.

## Documentation

For detailed documentation, including available functions and structures, refer to the [official documentation](https://docs.rs/nylas).

## Contributing

We welcome contributions from the open-source community! If you'd like to contribute to the Nylas Rust Client, please read our [Contributing Guidelines](CONTRIBUTING.md).

## License

This project is licensed under the [MIT License](LICENSE).
