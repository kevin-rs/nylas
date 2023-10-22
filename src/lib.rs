//! # Nylas Rust SDK
//!
//! The Nylas Rust SDK is a powerful library that simplifies interaction with the Nylas Email and Calendar API. With this SDK, you can seamlessly integrate Nylas services into your Rust applications, enabling you to work with email and calendar data efficiently.
//!
//! # Quick Start
//!
//! Get up and running with the Nylas Rust SDK quickly by following these simple steps:
//!
//! 1. Add the Nylas Rust SDK to your project's `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! nylas = "0.0.6"
//! tokio = "1.33.0"
//! ```
//!
//! 2. Import the library into your Rust code:
//!
//! ```rust
//! use nylas::client::Nylas;
//! ```
//!
//! 3. Initialize the Nylas client by providing your Nylas client ID, client secret, and access_token:
//!
//! ```rust
//! use nylas::client::Nylas;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client_id = "YOUR_CLIENT_ID";
//!     let client_secret = "YOUR_CLIENT_SECRET";
//!     let access_token = "YOUR_ACCESS_TOKEN";
//!     let mut nylas = Nylas::new(client_id, client_secret, Some(access_token)).await.unwrap();
//!     // ...
//! }
//! ```
//!
//! 4. Begin using Nylas services, including authentication, accessing account information, and managing messages.
//!
//! # Key Features
//!
//! The Nylas Rust SDK offers a range of features to simplify your integration with the Nylas Email and Calendar API:
//!
//! - **Simplified Authentication**: Quickly generate authentication URLs, easily exchange authorization codes for access tokens, and manage various authentication flows with simplicity.
//! - **Account Information Retrieval**: Retrieve comprehensive details about the Nylas account associated with your access token, including account metadata, plan details, and account status.
//! - **Message Management**: Seamlessly interact with email messages, allowing you to retrieve messages, send emails, filter messages based on criteria, and perform a wide range of message-related actions.
//!
//! # Usage
//!
//! ## Authentication
//!
//! ```rust
//! // Generate an authentication URL
//! let auth_url = nylas.authentication_url(redirect_uri, login_hint, state, scopes);
//!
//! // Exchange authorization code for access token
//! let access_token = "YOUR_ACCESS_TOKEN";
//! nylas = Nylas::new(client_id, client_secret, Some(access_token)).await.unwrap();
//! ```
//!
//! ## Account Information
//!
//! ```rust
//! // Access account details
//! println!("{:?}", nylas.account);
//! ```
//!
//! ## Message Management
//!
//! ```rust
//! // Retrieve all messages
//! let messages = nylas.messages().all().await;
//!
//! // Search for messages
//! let result = nylas.messages().search("example@example.com", Some(1), Some(0)).await;
//!
//! // Filter and retrieve messages
//! let filter = Some(hashmap!{
//!     "to" => "example@example.com"
//! });
//! let messages = nylas.messages().where_(filter, Some(View::Expanded)).await;
//!
//! // Retrieve the first message
//! let message_result = nylas.messages().first().await;
//!
//! // Get a specific message
//! let message_id = "YOUR_MESSAGE_ID";
//! let message_result = nylas.messages().get(message_id, Some(View::Expanded)).await;
//! ```
//!
//! # GitHub Repository
//!
//! You can access the source code for this library on [GitHub](https://github.com/wiseaidev/nylas).
//!
//! # Contributing
//!
//! We actively welcome contributions and bug reports from the community. If you'd like to contribute, report a bug, or suggest an enhancement, please feel free to engage with the project on [GitHub](https://github.com/wiseaidev/nylas). Your contributions are invaluable in making this library better for everyone.

pub mod accounts;
pub mod client;
pub mod messages;
