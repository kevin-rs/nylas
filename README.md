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
nylas = "0.0.6"
tokio = "1.33.0"
```

You can find the latest version on [Crates.io](https://crates.io/crates/nylas).

## Usage

Here's a simple example of how to retrieve all messages using the Nylas Rust Client:

```rust
use nylas::auth::Nylas;

#[tokio::main]
async fn main() {
    let client_id = "YOUR_CLIENT_ID";
    let client_secret = "YOUR_CLIENT_SECRET";
    
    // Initialize a Nylas client with client ID and client secret, without an access token
    let mut nylas = Nylas::new(client_id, client_secret, None).await.unwrap();

    let redirect_uri = "http://localhost:3000";
    let login_hint = Some("YOUR_EMAIL_ADDRESS");
    let state = Some("unique_identifier");
    let scopes = Some("email,calendar,contacts");

    // Generate an authentication URL for user login
    match nylas.authentication_url(redirect_uri, login_hint, state, scopes) {
        Ok(auth_url) => println!("Authentication URL: {}", auth_url),
        Err(error) => eprintln!("Error: {}", error),
    }

    // In a real-world app, the client will receive an authorization code from the user after login
    let authorization_code = "YOUR_AUTHORIZATION_CODE";
    
    // Exchange the authorization code for an access token
    match nylas.exchange_access_token(authorization_code).await {
        Ok(access_token) => println!("Access Token: {}", access_token),
        Err(error) => eprintln!("Error: {}", error),
    }

    // Reinitialize the Nylas client with the obtained access token
    let access_token = "YOUR_ACCESS_TOKEN";
    nylas = Nylas::new(client_id, client_secret, Some(access_token)).await.unwrap();

    // Retrieve and print the account information associated with the access token
    println!("Account Info: {:?}", nylas.account);

    // Call the all method to retrieve all messages
    let messages = nylas.messages().all().await;
    match messages {
        Ok(messages) => {
            for message in messages {
                // Process each message (Replace or supplement this part with your own logic)
                println!("{:?}", nylas.messages);
            }
        }
        Err(err) => {
            // Handle and print any errors while fetching messages
            eprintln!("Error: {}", err);
        }
    }
    
    // Access fields of a message, here printing the ID of the last message
    println!("Last message ID: {:?}", nylas.messages.unwrap().pop().unwrap().id);
}
```

TODO: Explore the [examples](examples) folder for more usage scenarios.

## Documentation

For detailed documentation, including available functions and structures, refer to the [official documentation](https://docs.rs/nylas).

## Contributing

We welcome contributions from the open-source community! If you'd like to contribute to the Nylas Rust Client, please read our [Contributing Guidelines](CONTRIBUTING.md).

## License

This project is licensed under the [MIT License](LICENSE).
