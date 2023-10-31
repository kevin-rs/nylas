# Nylas Rust SDK Rocket Demo

🚀This project is a demo of using the Nylas Rust SDK with the Rocket web framework.

## Getting Started

### Prerequisites

🔧 Before getting started, make sure you have the following prerequisites:

- Rust installed on your system.

### Installation

This project includes a `Makefile` to simplify the installation process. Ensure that you have the `make` utility available on your system.

1. Clone the project repository:

   ```shell
   git clone https://github.com/yourusername/nylas.git
   cd nylas/examples/rocket
   ```

1. Install the crate and all required core dependencies:

   ```shell
   make install
   ```

   If the `.env` file doesn't exist, this command creates it by copying from `.env.example`. This command also builds the project.

1. **Visit the Nylas Website:** Go to the official Nylas website at [https://dashboard.nylas.com/sign-in](https://dashboard.nylas.com/sign-in) to create a Nylas account if you don't already have one.

1. **Registration Process:** Complete the registration process by filling in the necessary information.

Now that you have your Nylas project set up, you need to set the Client ID, and Client Secret in your application:

1. **Set the Nylas Client ID, and Client Secret:** Open your project's configuration file or `.env` file, where you store environment variables.

1. **Add the Nylas Client ID , Client Secret, and System Token:** In your configuration file, add the following environment variables, replacing `<Nylas_Client_ID>`, and `<Nylas_Client_Secret>`, with the actual values you obtained from the Nylas dashboard:
     ```yaml
     # Nylas
     NYLAS_CLIENT_ID=<Your_Nylas_Client_ID>
     NYLAS_CLIENT_SECRET=<Your_Nylas_Client_Secret>
     ```
     - Save the changes to your configuration file.

1. Run the server locally:

   ```shell
   make run
   ```

## Usage

🚀 To use this project, follow the installation steps and then run the server locally.

## API Endpoints

This project provides the following API endpoints:

### Generate Authentication Token

This endpoint generates an authentication URL.

- **HTTP Method**: GET
- **Path**: `/nylas/generate-auth-token`

### Exchange Access Token

This endpoint exchanges an authorization code for an access token.

- **HTTP Method**: POST
- **Path**: `/nylas/exchange-access-token`

### Get Messages

This endpoint retrieves messages.

- **HTTP Method**: GET
- **Path**: `/nylas/messages`

#### All Messages

Retrieves all messages.

- **Path**: `/all`

#### First Message

Retrieves the first message.

- **Path**: `/first`

## cURL Examples

Here are cURL examples to interact with the endpoints:

### Generate Authentication Token (Example)

🔑 Generate an authentication token.

```shell
curl -X GET http://127.0.0.1:8000/nylas/generate-auth-token
```

### Exchange Access Token (Example)

🔄 Exchange an authorization code for an access token.

```shell
curl -X POST -H "Content-Type: application/json" -d 'your_authorization_code' http://127.0.0.1:8000/nylas/exchange-access-token
```

### Get All Messages (Example)

📬 Retrieve all messages.

```shell
curl -X GET -H "Authorization: YOUR_ACCESS_TOKEN" http://127.0.0.1:8000/nylas/messages/all
```

### Get First Message (Example)

📩 Retrieve the first message.

```shell
curl -X GET -H "Authorization: YOUR_ACCESS_TOKEN" http://127.0.0.1:8000/nylas/recent-message
```

## License

📜 This project is licensed under the [MIT](LICENSE) license - see the [LICENSE](LICENSE) file for details.
