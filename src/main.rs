use std::{env, fmt::format, net::SocketAddr};

use axum::{body::Bytes, extract::State, routing::post, Router};
use http::{HeaderMap, StatusCode};
use tokio::net::TcpListener;
use traq::apis::configuration::Configuration;
use traq_bot_http::{Event, RequestParser};

mod akinator;

#[derive(Clone)]
struct App {
    request_parser: RequestParser,
    client_config: Configuration,
}
#[tokio::main]
async fn main() {
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
    let router = Router::new().route("/", post(handler)).with_state(app);
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

async fn handler(State(app): State<App>, headers: HeaderMap, body: Bytes) -> StatusCode {
    match app.request_parser.parse(headers.iter(), &body) {
        Ok(Event::MessageCreated(payload)) => {
            use traq::apis::message_api::post_message;
            let channel_id = payload.message.channel_id;
            let user = payload.message.user;
            println!(
                "{}さんにチャンネルID : {} で返答",
                user.display_name, channel_id
            );
            let reply_content = if payload.message.plain_text.contains("homeru") {
                format!("えらい～～～～！！！！！！！!").to_string()
            } else {
                format!("@{} おいす～！", user.name).to_string()
            };
            let request = traq::models::PostMessageRequest {
                content: reply_content,
                embed: Some(true),
            };
            let res = post_message(&app.client_config, &channel_id, Some(request)).await;
            if let Err(e) = res {
                eprintln!("Error: {e}");
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
            StatusCode::NO_CONTENT
        }
        Ok(Event::DirectMessageCreated(payload)) => {
            use traq::apis::message_api::post_direct_message;
            let user = payload.message.user;
            println!("{}さんにDMで返答", user.display_name);
            let reply_content = if payload.message.plain_text.contains("homeru") {
                format!("えらい～～～～！！！！！！！!").to_string()
            } else {
                format!("@{} おいす～！", user.name).to_string()
            };
            let request = traq::models::PostMessageRequest {
                content: reply_content,
                embed: Some(true),
            };
            let res = post_direct_message(&app.client_config, &user.id, Some(request)).await;
            if let Err(e) = res {
                eprintln!("Error: {e}");
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
            StatusCode::NO_CONTENT
        }
        Ok(_) => StatusCode::NO_CONTENT,
        Err(err) => {
            eprintln!("{err}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
