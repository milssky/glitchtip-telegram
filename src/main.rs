use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{error, info};
use tracing_subscriber;

mod templates;
use crate::templates::format_payload;

// Configuration loaded from CLI args and environment.
#[derive(clap::Parser)]
struct Config {
    // Telegram bot token for sending messages.
    #[arg(long, env)]
    bot_token: String,
    // Target chat or channel ID to send notifications to.
    #[arg(long, env)]
    chat_id: String,
    // Port for the webhook HTTP server.
    #[arg(long, env)]
    port: u16,
    // Webhook path
    #[arg(long, env, default_value = "webhook")]
    webhook: String
}

#[derive(Clone)]
struct AppState {
    client: reqwest::Client,
    send_message_url: String,
    chat_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct Field {
    title: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct Attachment {
    title: String,
    title_link: Option<String>,
    text: Option<String>,
    fields: Option<Vec<Field>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct Payload {
    text: String,
    attachments: Vec<Attachment>,
}

async fn send_message(state: &AppState, text: &str) -> Result<(), reqwest::Error> {
    let body = json!({
        "chat_id": state.chat_id,
        "text": text,
        "parse_mode": "HTML",
    });

    let response = state
        .client
        .post(&state.send_message_url)
        .json(&body)
        .send()
        .await?;

    response.error_for_status()?;
    Ok(())
}

// Handle incoming Glitchtip webhook
async fn webhook(
    State(state): State<AppState>,
    Json(payload): Json<Payload>,
) -> (StatusCode, impl IntoResponse) {
    info!("Processing webhook");

    // Build Telegram message from payload text and attachments.
    let message = format_payload(&payload);

    // Send to Telegram; log on failure but still return success to Glitchtip.
    if let Err(err) = send_message(&state, &message).await {
        error!("Failed to send Telegram message: {}", err);
    }
    (StatusCode::OK, Json(json!({ "status": "ok" })))
}

#[tokio::main]
async fn main() {
    // Initialize logging.
    tracing_subscriber::fmt::init();

    // Parse our configuration from the environment.
    // This will exit with a help message if something is wrong.
    let config = Config::parse();

    // Create shared application state for handlers.
    let app_state = AppState {
        client: reqwest::Client::new(),
        send_message_url: format!(
            "https://api.telegram.org/bot{}/sendMessage",
            config.bot_token
        ),
        chat_id: config.chat_id,
    };

    // Build HTTP routes; `webhook` will handle incoming JSON payloads.
    let webhook_path = format!("/{}", config.webhook);
    let app = Router::new()
        .route(&webhook_path, post(webhook))
        .with_state(app_state);

    // Construct the address to listen at the specified port.
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], config.port));
    info!("Server started on {}", addr);

    // Bind TCP listener and run the Axum server.
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
