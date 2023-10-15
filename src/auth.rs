use std::collections::HashMap;
use url::Url;

/// The `Nylas` struct provides all methods available in the Nylas API.
///
/// This struct currently allows you to create authentication URLs for initiating the OAuth 2.0 flow with the Nylas API.
///
/// # Examples
///
/// To create a new `Nylas` instance with your client ID and client secret:
///
/// ```
/// use nylas::auth::Nylas;
///
/// let client_id = "YOUR_CLIENT_ID";
/// let client_secret = "YOUR_CLIENT_SECRET";
///
/// let nylas = Nylas::new(client_id, client_secret);
/// ```
pub struct Nylas {
    pub client_id: String,
    pub client_secret: String,
}

impl Nylas {
    /// Create a new `Nylas` instance with the provided client ID and client secret.
    ///
    /// # Arguments
    ///
    /// * `client_id` - A string representing your Nylas API client ID.
    /// * `client_secret` - A string representing your Nylas API client secret.
    ///
    /// # Examples
    ///
    /// ```
    /// use nylas::auth::Nylas;
    ///
    /// let client_id = "YOUR_CLIENT_ID";
    /// let client_secret = "YOUR_CLIENT_SECRET";
    ///
    /// let nylas = Nylas::new(client_id, client_secret);
    /// ```
    pub fn new(client_id: &str, client_secret: &str) -> Self {
        Nylas {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
        }
    }

    /// Generate an authentication URL for initiating the OAuth 2.0 flow.
    ///
    /// The authentication URL can be opened in a web browser to allow users to grant
    /// permission to your application.
    ///
    /// # Arguments
    ///
    /// * `redirect_uri` - The URL to which the user will be redirected after authentication.
    /// * `login_hint` - An optional hint to pre-fill the user's email address on the authentication page.
    /// * `state` - An optional unique identifier for the authentication request, which can be used to maintain state during the flow.
    /// * `scopes` - An optional list of scopes that specify the permissions your application is requesting.
    ///
    /// # Returns
    ///
    /// A `Result` containing the authentication URL if successful, or an error message.
    ///
    /// # Errors
    ///
    /// Returns an error if any of the following conditions are not met:
    /// 1. The client ID and client secret are not provided.
    /// 2. The redirect URI is not a valid URL.
    ///
    /// # Examples
    ///
    /// ```
    /// use nylas::auth::Nylas;
    ///
    /// let client_id = "YOUR_CLIENT_ID";
    /// let client_secret = "YOUR_CLIENT_SECRET";
    ///
    /// let nylas = Nylas::new(client_id, client_secret);
    ///
    /// let redirect_uri = "http://example.com/login_callback";
    /// let login_hint = Some("your_email@example.com");
    /// let state = Some("unique_identifier");
    /// let scopes = Some("email,calendar,contacts");
    ///
    /// match nylas.authentication_url(redirect_uri, login_hint, state, scopes) {
    ///     Ok(auth_url) => println!("Authentication URL: {}", auth_url),
    ///     Err(error) => eprintln!("Error: {}", error),
    /// }
    /// ```
    pub fn authentication_url(
        &self,
        redirect_uri: &str,
        login_hint: Option<&str>,
        state: Option<&str>,
        scopes: Option<&str>,
    ) -> Result<String, String> {
        if self.client_id.is_empty() || self.client_secret.is_empty() {
            return Err("Client ID and Client Secret must not be empty.".to_string());
        }

        if !Url::parse(redirect_uri).is_ok() {
            return Err("Invalid redirect URI.".to_string());
        }

        let mut params: HashMap<&str, String> = HashMap::new();
        params.insert("client_id", self.client_id.clone());
        params.insert("redirect_uri", redirect_uri.to_string());
        params.insert("response_type", "code".to_string());

        if let Some(login_hint) = login_hint {
            params.insert("login_hint", login_hint.to_string());
        }

        if let Some(state) = state {
            params.insert("state", state.to_string());
        }

        if let Some(scopes) = scopes {
            params.insert("scopes", scopes.to_string());
        }

        // Build the URL
        let base_url = "https://api.nylas.com/oauth/authorize";
        let mut url = String::from(base_url);
        url.push('?');

        for (key, value) in params.iter() {
            url.push_str(key);
            url.push_str("=");
            url.push_str(value);
            url.push('&');
        }

        // Remove the trailing '&' character
        url.pop();

        Ok(url)
    }

    /// Exchange the authorization code for an access token using hosted authentication.
    ///
    /// The authorization code is valid for 15 minutes and can be used only once.
    ///
    /// # Arguments
    ///
    /// * `authorization_code` - The authorization code obtained during the authentication process.
    ///
    /// # Returns
    ///
    /// A `Result` containing the access token if successful, or an error message.
    ///
    /// # Errors
    ///
    /// Returns an error if any of the following conditions are not met:
    /// 1. The client ID and client secret are not provided.
    /// 2. The `authorization_code` is not valid.
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
    ///
    ///     let nylas = Nylas::new(client_id, client_secret);
    ///
    ///     let authorization_code = "YOUR_AUTHORIZATION_CODE";
    ///
    ///     match nylas.exchange_access_token(authorization_code).await {
    ///         Ok(access_token) => println!("Access Token: {}", access_token),
    ///         Err(error) => eprintln!("Error: {}", error),
    ///     }
    /// }
    /// ```
    pub async fn exchange_access_token(&self, authorization_code: &str) -> Result<String, String> {
        if self.client_id.is_empty() || self.client_secret.is_empty() {
            return Err("Client ID and Client Secret must not be empty.".to_string());
        }

        let mut params: HashMap<&str, String> = HashMap::new();
        params.insert("client_id", self.client_id.clone());
        params.insert("client_secret", self.client_secret.clone());
        params.insert("grant_type", "authorization_code".to_string());
        params.insert("code", authorization_code.to_string());

        // Build the URL
        let base_url = "https://api.nylas.com/oauth/token";

        // Make the POST request
        let client = reqwest::Client::new();
        let response = client
            .post(base_url)
            .header("Accept", "application/json")
            .form(&params)
            .send()
            .await
            .map_err(|e| format!("Request Error: {:?}", e))?;

        if response.status().is_success() {
            let data: HashMap<String, String> = response
                .json()
                .await
                .map_err(|e| format!("JSON Parsing Error: {:?}", e))?;
            if let Some(access_token) = data.get("access_token") {
                return Ok(access_token.to_string());
            } else {
                return Err("Access token not found in the response.".to_string());
            }
        } else {
            return Err(format!("HTTP Error: {}", response.status()));
        }
    }
}
