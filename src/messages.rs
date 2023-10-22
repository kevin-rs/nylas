use crate::client::Nylas;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Represents an email view.
#[derive(Debug)]
pub enum View {
    Ids,
    Count,
    Expanded,
}

impl ToString for View {
    fn to_string(&self) -> String {
        match self {
            View::Ids => "ids".to_string(),
            View::Count => "count".to_string(),
            View::Expanded => "expanded".to_string(),
        }
    }
}

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
    // handle string or null while deserializing the response.
    pub headers: Option<Value>,
}

impl Message {
    /// Checks if a message matches a given filter based on its attributes.
    ///
    /// # Arguments
    ///
    /// - `self`: A reference to the `Message` struct.
    /// - `filter`: A hashmap containing filtering criteria as key-value pairs.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the message matches the filter.
    ///
    /// # Filtering Criteria
    ///
    /// The filter hashmap can include the following criteria:
    ///
    /// - `"to"`: Check if the message is sent to a specific email address.
    /// - `"from"`: Check if the message is sent from a specific email address.
    /// - `"cc"`: Check if the message includes a specific email address in the CC field.
    /// - `"bcc"`: Check if the message includes a specific email address in the BCC field.
    /// - `"date"`: Check if the message's date matches a specific Unix timestamp.
    /// - `"unread"`: Check if the message is marked as unread (true or false).
    /// - `"starred"`: Check if the message is marked as starred (true or false).
    /// - `"snippet"`: Check if the message's snippet contains a specific keyword.
    /// - `"subject"`: Check if the message's subject contains a specific keyword.
    /// - `"body"`: Check if the message's body contains a specific keyword.
    /// - `"thread_id"`: Check if the message belongs to a specific thread (by ID).
    /// - `"labels"`: Check if the message is labeled with specific labels (comma-separated).
    fn matches_filter(&self, filter: &HashMap<&str, &str>) -> bool {
        if let Some(to) = filter.get("to") {
            if !self
                .to
                .iter()
                .any(|recipient| recipient.email == to.to_string())
            {
                return false;
            }
        }

        if let Some(from) = filter.get("from") {
            if !self
                .from
                .iter()
                .any(|sender| sender.email == from.to_string())
            {
                return false;
            }
        }

        if let Some(cc) = filter.get("cc") {
            if !self
                .cc
                .iter()
                .any(|recipient| recipient.email == cc.to_string())
            {
                return false;
            }
        }

        if let Some(bcc) = filter.get("bcc") {
            if !self
                .bcc
                .iter()
                .any(|recipient| recipient.email == bcc.to_string())
            {
                return false;
            }
        }

        if let Some(date_str) = filter.get("date") {
            let filter_date = date_str.parse::<i64>().unwrap_or(0);
            if self.date != filter_date {
                return false;
            }
        }

        if let Some(unread_str) = filter.get("unread") {
            let filter_unread = unread_str.parse::<bool>().unwrap_or(false);
            if self.unread != filter_unread {
                return false;
            }
        }

        if let Some(starred_str) = filter.get("starred") {
            let filter_starred = starred_str.parse::<bool>().unwrap_or(false);
            if self.starred != filter_starred {
                return false;
            }
        }

        if let Some(snippet) = filter.get("snippet") {
            if !self.snippet.contains(snippet) {
                return false;
            }
        }

        if let Some(subject) = filter.get("subject") {
            if !self.subject.contains(subject) {
                return false;
            }
        }

        if let Some(body) = filter.get("body") {
            if !self.body.contains(body) {
                return false;
            }
        }

        if let Some(thread_id) = filter.get("thread_id") {
            if self.thread_id != thread_id.to_string() {
                return false;
            }
        }

        if let Some(labels) = filter.get("labels") {
            let filter_labels: Vec<&str> = labels.split(',').collect();
            if !filter_labels
                .iter()
                .any(|label| self.labels.iter().any(|l| l.name == label.to_string()))
            {
                return false;
            }
        }

        // TODO: Add more filtering logic for other attributes
        true
    }
}

/// Struct for working with Nylas messages.
pub struct Messages<'a> {
    pub nylas: &'a mut Nylas,
}

impl<'a> Messages<'a> {
    pub fn new(nylas: &'a mut Nylas) -> Self {
        Messages { nylas }
    }
    /// Retrieve all messages from the Nylas API.
    ///
    /// # Arguments
    ///
    /// - `self`: A mutable reference to the `Messages` struct.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of messages if successful, or an error message.
    ///
    /// # Errors
    ///
    /// This method can return an error if the access token is not provided or if the request to the Nylas API fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nylas::client::Nylas;
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
    /// # }
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
                    // self.nylas.messages = Some(messages.clone());
                    Ok(messages)
                } else {
                    Err(format!("Request failed with status: {}", response.status()))
                }
            }
            Err(err) => Err(err.to_string()),
        }
    }

    /// This method allows you to search for messages based on a query string, with
    /// optional limits and offsets to paginate the results.
    ///
    /// # Arguments
    ///
    /// - `self`: A mutable reference to the `Messages` struct.
    /// - `query`: A search query string.
    /// - `limit`: An optional limit to specify the number of results to retrieve.
    /// - `offset`: An optional offset to specify the starting point of results.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of messages if successful, or an error message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use nylas::client::Nylas;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client_id = "YOUR_CLIENT_ID";
    ///     let client_secret = "YOUR_CLIENT_SECRET";
    ///     let access_token = "YOUR_ACCESS_TOKEN";
    ///
    ///     let mut nylas = Nylas::new(client_id, client_secret, Some(access_token)).await.unwrap();
    ///
    ///     Call the `search` method to search for messages with a limit and offset
    ///     let result = nylas.messages().search("yo.code.inbox@gmail.com", Some(1), Some(0)).await;
    ///     match result {
    ///         Ok(messages) => {
    ///             for message in messages {
    ///                 // Process each searched message
    ///                 println!("{:?}", message);
    ///             }
    ///         }
    ///         Err(err) => {
    ///             // Handle the error
    ///             eprintln!("Error: {}", err);
    ///         }
    ///     }
    /// }
    /// ```
    pub async fn search(
        &mut self,
        query: &str,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Vec<Message>, String> {
        // Construct the API URL with the search query, limit, and offset
        let mut url = format!("https://api.nylas.com/messages/search?q={}", query);

        if let Some(limit) = limit {
            url.push_str(&format!("&limit={}", limit));
        }

        if let Some(offset) = offset {
            url.push_str(&format!("&offset={}", offset));
        }

        // Create an HTTP client with the bearer token in the headers
        let client = reqwest::Client::new();
        let request = client
            .get(&url)
            .header("Accept", "application/json")
            .header(
                "Authorization",
                format!(
                    "Bearer {}",
                    self.nylas
                        .access_token
                        .as_ref()
                        .ok_or("Access token not provided")?,
                ),
            )
            .send();

        // Handle the HTTP response
        match request.await {
            Ok(response) => {
                if response.status().is_success() {
                    // Parse the JSON response into a vector of Message
                    let messages: Vec<Message> = response.json().await.unwrap();
                    Ok(messages)
                } else {
                    Err(format!("Request failed with status: {}", response.status()))
                }
            }
            Err(err) => Err(err.to_string()),
        }
    }

    /// Filters messages based on specified criteria with an optional view parameter.
    ///
    /// # Arguments
    ///
    /// - `self`: A mutable reference to the `Messages` struct.
    /// - `filter`: An optional hashmap containing filtering criteria as key-value pairs.
    /// - `view`: An optional view parameter (enum) for the message. Allowed values: ids, count, expanded.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of messages that match the filter if successful, or an error message.
    ///
    /// # Errors
    ///
    /// This method can return an error if the access token is not provided, if the request to the Nylas API fails when calling the `all` method, or if there's an issue filtering the messages.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nylas::client::Nylas;
    /// use nylas::messages::View;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client_id = "YOUR_CLIENT_ID";
    ///     let client_secret = "YOUR_CLIENT_SECRET";
    ///     let access_token = "YOUR_ACCESS_TOKEN";
    ///
    ///     let mut nylas = Nylas::new(client_id, client_secret, Some(access_token)).await.unwrap();
    ///
    ///     Define filter parameters as a HashMap
    ///     let mut filter = HashMap::new();
    ///     filter.insert("to", "oss@wiseai.dev");
    ///     
    ///     Call the `where_` method with filter and view parameters
    ///     let result = nylas.messages().where_(Some(filter), Some(View::Expanded)).await;
    ///     match result {
    ///         Ok(messages) => {
    ///             // Process the filtered messages
    ///             for message in messages {
    ///                 println!("Filtered Message: {:?}", message);
    ///             }
    ///         }
    ///         Err(err) => {
    ///             eprintln!("Error: {}", err);
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// # Filtering Criteria
    ///
    /// The filter hashmap can include various criteria for filtering messages. The available filtering criteria include:
    ///
    /// - `"to"`: Filter messages that are sent to a specific email address.
    /// - `"from"`: Filter messages that are sent from a specific email address.
    /// - `"cc"`: Filter messages that include a specific email address in the CC field.
    /// - `"bcc"`: Filter messages that include a specific email address in the BCC field.
    /// - `"date"`: Filter messages with a specific Unix timestamp.
    /// - `"unread"`: Filter messages marked as unread (true or false).
    /// - `"starred"`: Filter messages marked as starred (true or false).
    /// - `"snippet"`: Filter messages with a snippet containing a specific keyword.
    /// - `"subject"`: Filter messages with a subject containing a specific keyword.
    /// - `"body"`: Filter messages with a body containing a specific keyword.
    /// - `"thread_id"`: Filter messages belonging to a specific thread (by ID).
    /// - `"labels"`: Filter messages with specific labels (comma-separated).
    pub async fn where_(
        &mut self,
        filter: Option<HashMap<&str, &str>>,
        view: Option<View>,
    ) -> Result<Vec<Message>, String> {
        // Call the `all` method to retrieve all messages
        let mut url = "https://api.nylas.com/messages".to_string();

        if let Some(view) = view {
            url.push_str(&format!("?view={}", view.to_string()));
        }

        // Create an HTTP client with the bearer token in the headers
        let client = reqwest::Client::new();
        let request = client
            .get(&url)
            .header("Accept", "application/json")
            .header(
                "Authorization",
                format!(
                    "Bearer {}",
                    self.nylas
                        .access_token
                        .as_ref()
                        .ok_or("Access token not provided")?,
                ),
            )
            .send();

        // Handle the HTTP response
        match request.await {
            Ok(response) => {
                if response.status().is_success() {
                    // Parse the JSON response into a vector of Message
                    let messages: Vec<Message> = response.json().await.unwrap();
                    // Filter messages based on the provided parameters
                    let filtered_messages: Vec<Message> = match filter {
                        Some(filter) => messages
                            .into_iter()
                            .filter(|message| message.matches_filter(&filter))
                            .collect(),
                        None => messages,
                    };
                    Ok(filtered_messages)
                } else {
                    Err(format!("Request failed with status: {}", response.status()))
                }
            }
            Err(err) => Err(err.to_string()),
        }
    }

    /// Retrieve the most recent message from the Nylas API.
    ///
    /// # Arguments
    ///
    /// - `self`: A mutable reference to the `Messages` struct.
    ///
    /// # Returns
    ///
    /// A `Result` containing an `Option<Message>` if successful (Some(message)), or an error message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nylas::client::Nylas;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client_id = "YOUR_CLIENT_ID";
    ///     let client_secret = "YOUR_CLIENT_SECRET";
    ///     let access_token = "YOUR_ACCESS_TOKEN";
    ///
    ///     let mut nylas = Nylas::new(client_id, client_secret, Some(access_token)).await.unwrap();
    ///
    ///     let message_result = nylas.messages().first().await;
    ///     match message_result {
    ///         Ok(Some(message)) => {
    ///             // Process the first message
    ///             println!("{:?}", message);
    ///         }
    ///         Ok(None) => {
    ///             // Handle the case when there are no messages
    ///             println!("No messages found.");
    ///         }
    ///         Err(err) => {
    ///             // Handle the error
    ///             eprintln!("Error: {}", err);
    ///         }
    ///     }
    /// }
    /// ```
    /// # Errors
    ///
    /// This method can return an error if the access token is not provided, or if the request to the Nylas API fails.
    pub async fn first(&mut self) -> Result<Option<Message>, String> {
        // Retrieve all messages
        let all_messages = self.all().await?;

        // Sort the messages by the "date" field in descending order (most recent first)
        let mut sorted_messages = all_messages.clone();
        sorted_messages.sort_by(|a, b| b.date.cmp(&a.date));

        // If there are messages, return the first one; otherwise, return None
        if !sorted_messages.is_empty() {
            Ok(Some(sorted_messages[0].clone()))
        } else {
            Ok(None)
        }
    }

    /// Retrieve a specific message by its ID with an optional view parameter.
    ///
    /// # Arguments
    ///
    /// - `self`: A mutable reference to the `Messages` struct.
    /// - `id`: The ID of the message you want to retrieve.
    /// - `view`: An optional view parameter (enum) for the message. Allowed values: ids, count, expanded.
    ///
    /// # Returns
    ///
    /// A `Result` containing an `Option<Message>` if successful (Some(message)), or an error message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nylas::client::Nylas;
    /// use nylas::messages::View;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client_id = "YOUR_CLIENT_ID";
    ///     let client_secret = "YOUR_CLIENT_SECRET";
    ///     let access_token = "YOUR_ACCESS_TOKEN";
    ///
    ///     let mut nylas = Nylas::new(client_id, client_secret, Some(access_token)).await.unwrap();
    ///
    ///     Retrieve a specific message by ID with a view parameter
    ///     let message_id = "your_message_id_here";
    ///     let message_result = nylas.messages().get(message_id, Some(View::Expanded)).await;
    ///     match message_result {
    ///         Ok(Some(message)) => {
    ///             // Process the retrieved message
    ///             println!("{:?}", message);
    ///         }
    ///         Ok(None) => {
    ///             // Handle the case when the message is not found
    ///             println!("Message not found.");
    ///         }
    ///         Err(err) => {
    ///             // Handle the error
    ///             eprintln!("Error: {}", err);
    ///         }
    ///     }
    /// }
    /// ```
    pub async fn get(&mut self, id: &str, view: Option<View>) -> Result<Option<Message>, String> {
        // Construct the API URL for the specific message with view parameter
        let mut url = format!("https://api.nylas.com/messages/{}", id);

        if let Some(view) = view {
            url.push_str(&format!("?view={}", view.to_string()));
        }

        // Create an HTTP client with the bearer token in the headers
        let client = reqwest::Client::new();
        let request = client
            .get(&url)
            .header("Accept", "application/json")
            .header(
                "Authorization",
                format!(
                    "Bearer {}",
                    self.nylas
                        .access_token
                        .as_ref()
                        .ok_or("Access token not provided")?,
                ),
            )
            .send();

        // Handle the HTTP response
        match request.await {
            Ok(response) => {
                if response.status().is_success() {
                    // Parse the JSON response into a message or an empty response
                    let message: Option<Message> = response.json().await.unwrap_or_default();
                    Ok(message)
                } else if response.status() == reqwest::StatusCode::NOT_FOUND {
                    Ok(None)
                } else {
                    Err(format!("Request failed with status: {}", response.status()))
                }
            }
            Err(err) => Err(err.to_string()),
        }
    }
}
