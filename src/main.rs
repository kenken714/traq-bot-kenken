use axum::{routing::post, Router};
use http::StatusCode;
use std::{env, net::SocketAddr};
use tokio::net::TcpListener;
use traq::apis::configuration::Configuration;
use traq_bot_http::RequestParser;

use akinator;

mod repository;
mod router;

#[derive(Clone)]
struct App {
    request_parser: RequestParser,
    client_config: Configuration,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let verification_token = env::var("VERIFICATION_TOKEN").unwrap();
    let bot_access_token = env::var("BOT_ACCESS_TOKEN").unwrap();

    let request_parser = RequestParser::new(&verification_token);
    let client_config = Configuration {
        bearer_access_token: Some(bot_access_token),
        ..Default::default()
    };
    let app = App {
        request_parser,
        client_config,
    };
    let infra = repository::Repository::connect().await?;

    let router = Router::new().route("/", post(handler)).with_state(app);
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, router).await?;

    Ok(())
}

async fn handler() -> (StatusCode, String) {
    (StatusCode::OK, "Hello, world!".to_string())
}
