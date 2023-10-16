//! Nylas Rust SDK
//!
//! The Nylas Rust SDK is a library that simplifies interaction with the Nylas Email and Calendar API.
//! It allows you to integrate Nylas services into your Rust applications with ease.
//!
//! # Quick Start
//!
//! To quickly get started with the Nylas Rust SDK, follow these steps:
//!
//! 1. Include the Nylas Rust SDK in your project's `Cargo.toml` file:
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
//! use nylas::auth::Nylas;
//! ```
//!
//! 3. Initialize the Nylas client by providing your Nylas client ID and client secret:
//!
//! ```rust
//! use nylas::auth::Nylas;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client_id = "YOUR_CLIENT_ID";
//!     let client_secret = "YOUR_CLIENT_SECRET";
//!     let mut nylas = Nylas::new(client_id, client_secret, None).await.unwrap();
//!     // ...
//! }
//! ```
//!
//! 4. Start using Nylas services, such as authentication, accessing account information, and fetching messages.
//!
//! # Features
//!
//! - Simplified Authentication: Easily generate authentication URLs, exchange authorization codes for access tokens, and more.
//! - Retrieve Account Information: Fetch details about the Nylas account associated with the access token.
//! - Message Management: Interact with email messages, including fetching all messages, sending emails, and more.
//!
//! # GitHub Repository
//!
//! The source code for this library can be found on [GitHub](https://github.com/wiseaidev/nylas).
//!
//! # Contributing
//!
//! We welcome contributions and bug reports. Please feel free to contribute to the project on [GitHub](https://github.com/wiseaidev/nylas).
//! Your contributions help make this library better for everyone.

pub mod accounts;
pub mod auth;
pub mod messages;
