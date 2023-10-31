#[macro_use]
extern crate rocket;
#[macro_use]
extern crate dotenv_codegen;
use dotenv;
use nylas::client::Nylas;
use nylas::messages::Message;
use rocket::http::{Method, Status};
use rocket::request::Outcome;
use rocket::serde::json::Json;
use rocket::{request::FromRequest, Request, State};
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use std::sync::Arc;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv::dotenv().ok();
    let allowed_origins = AllowedOrigins::all();
    let allowed_methods = vec![
        Method::Get,
        Method::Post,
        Method::Options,
        Method::Put,
        Method::Delete,
    ]
    .into_iter()
    .map(From::from)
    .collect();
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: allowed_methods,
        allowed_headers: AllowedHeaders::all(),
        ..rocket_cors::CorsOptions::default()
    }
    .to_cors()
    .expect("Error in CORS setup");

    let client_id = dotenv!("NYLAS_CLIENT_ID");
    let client_secret = dotenv!("NYLAS_CLIENT_SECRET");

    let client = Nylas::new(client_id, client_secret, None).await.unwrap();
    let client_arc = Arc::new(client);
    let routes = all_routes();
    rocket::build()
        .mount("/", routes)
        .attach(cors)
        .manage(client_arc)
        .launch()
        .await
        .expect("Launch Error");
    Ok(())
}

fn all_routes() -> Vec<rocket::Route> {
    routes![
        generate_auth_token,
        exchange_access_token,
        get_all_messages,
        get_first_message
    ]
}

#[get("/nylas/generate-auth-token")]
fn generate_auth_token(client: &State<Arc<Nylas>>) -> String {
    // Define authentication parameters
    let login_hint = Some("mahmoudddharmouchhh@gmail.com");
    let state = Some("unique_identifier");
    let scopes = Some("email,calendar,contacts");

    // Generate an authentication URL
    match client.authentication_url(dotenv!("NYLAS_CLIENT_URI"), login_hint, state, scopes) {
        Ok(auth_url) => auth_url,
        Err(error) => error,
    }
}

#[post(
    "/nylas/exchange-access-token",
    format = "application/json",
    data = "<authorization_code>"
)]
async fn exchange_access_token(client: &State<Arc<Nylas>>, authorization_code: String) -> String {
    match client.exchange_access_token(&authorization_code).await {
        Ok(access_token) => access_token,
        Err(error) => error,
    }
}

#[derive(Debug)]
struct AccessToken {
    token: String,
}

#[async_trait]
impl<'r> FromRequest<'r> for AccessToken {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token_header: Option<String> = req
            .headers()
            .get_one("Authorization")
            .map(|s| s.to_string());

        match token_header {
            Some(token) => Outcome::Success(AccessToken { token }),
            None => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}

#[get("/nylas/messages")]
async fn get_all_messages(auth: AccessToken, client: &State<Arc<Nylas>>) -> Json<Vec<Message>> {
    let mut client_with_token =
        Nylas::new(&client.client_id, &client.client_secret, Some(&auth.token))
            .await
            .unwrap();

    // Call the all method to retrieve all messages
    let messages = client_with_token.messages().all().await;

    match messages {
        Ok(messages) => Json(messages),
        Err(_err) => todo!(),
    }
}

#[get("/nylas/recent-message")]
async fn get_first_message(auth: AccessToken, client: &State<Arc<Nylas>>) -> Json<Message> {
    let mut client_with_token =
        Nylas::new(&client.client_id, &client.client_secret, Some(&auth.token))
            .await
            .unwrap();

    let message_result = client_with_token.messages().first().await;

    match message_result {
        Ok(Some(message)) => Json(message),
        Ok(None) => todo!(),
        Err(_err) => todo!(),
    }
}
