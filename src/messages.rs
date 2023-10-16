use crate::auth::Nylas;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
/// Represents an email address with an optional name.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmailAddress {
    pub email: String,
    pub name: Option<String>,
}

/// Represents a file with content details.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct File {
    pub content_disposition: String,
    pub content_type: String,
    pub filename: Option<String>,
    pub id: String,
    pub size: i64,
}

/// Represents an event, such as a calendar event.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Event {
    pub account_id: String,
    pub busy: bool,
    pub calendar_id: String,
    pub description: String,
    pub id: String,
    pub location: String,
    pub message_id: String,
    pub object: String,
    pub owner: String,
    pub participants: Vec<Participant>,
    pub read_only: bool,
    pub reminders: Option<Value>,
    pub status: String,
    pub title: String,
    pub visibility: Option<Value>,
    pub when: HashMap<String, i64>,
}

/// Represents a participant in an event.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Participant {
    pub comment: Option<String>,
    pub email: String,
    pub name: String,
    pub phone_number: Option<String>,
    pub status: String,
}

/// Represents a folder.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Folder {
    // TODO
}

/// Represents a label.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Label {
    pub display_name: String,
    pub id: String,
    pub name: String,
}

/// Represents a message.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: String,
    pub object: String,
    pub account_id: String,
    pub thread_id: String,
    pub subject: String,
    pub from: Vec<EmailAddress>,
    pub to: Vec<EmailAddress>,
    pub cc: Vec<EmailAddress>,
    pub bcc: Vec<EmailAddress>,
    pub reply_to: Vec<EmailAddress>,
    pub date: i64,
    pub unread: bool,
    pub starred: bool,
    pub snippet: String,
    pub body: String,
    pub files: Vec<File>,
    pub events: Vec<Event>,
    pub folder: Option<Folder>,
    pub labels: Vec<Label>,
}

/// Represents an Nylas account.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub id: String,
    pub object: String,
    pub account_id: String,
    pub name: String,
    pub provider: String,
    pub organization_unit: String,
    pub sync_state: String,
    pub linked_at: i32,
    pub email_address: String,
}

/// Struct for working with Nylas messages.
pub struct Messages<'a> {
    pub nylas: &'a mut Nylas,
}

impl<'a> Messages<'a> {
    /// Retrieve all messages from the Nylas API.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of messages if successful, or an error message.
    ///
    /// # Examples
    ///
    /// ```
    /// use nylas::auth::Nylas;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client_id = "YOUR_CLIENT_ID";
    ///     let client_secret = "YOUR_CLIENT_SECRET";
    ///     let access_token = "YOUR_ACCESS_TOKEN";
    ///
    ///     // let mut nylas = Nylas::new(client_id, client_secret, Some(access_token)).await.unwrap();
    ///
    ///     // Call the all method to retrieve all messages
    ///     // let messages = nylas.messages().all().await;
    ///     // match messages {
    ///     //     Ok(messages) => {
    ///     //         for message in messages {
    ///     //             // Process each message
    ///     //             println!("{:?}", nylas.messages);
    ///     //         }
    ///     //     }
    ///     //     Err(err) => {
    ///     //         // Handle the error
    ///     //         eprintln!("Error: {}", err);
    ///     //     }
    ///     // }
    /// }
    /// ```
    pub async fn all(&mut self) -> Result<Vec<Message>, String> {
        // Construct the API URL
        let url = "https://api.nylas.com/messages";

        // Create an HTTP client with the bearer token in the headers
        let client = reqwest::Client::new();
        let request = client
            .get(url)
            .header("Accept", "application/json")
            .header(
                "Authorization",
                format!(
                    "Bearer {}",
                    self.nylas
                        .access_token
                        .as_ref()
                        .ok_or("Access token not provided")?
                ),
            )
            .send();

        // Handle the HTTP response
        match request.await {
            Ok(response) => {
                if response.status().is_success() {
                    // Parse the JSON response into a vector of Message
                    let messages: Vec<Message> = response.json().await.unwrap();
                    // Set the messages attribute
                    self.nylas.messages = Some(messages.clone());
                    Ok(messages)
                } else {
                    Err(format!("Request failed with status: {}", response.status()))
                }
            }
            Err(err) => Err(err.to_string()),
        }
    }
}
