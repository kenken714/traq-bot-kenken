use axum::{body::Bytes, extract::State, routing::post, Router};
use http::{HeaderMap, StatusCode};
use std::{env, net::SocketAddr};
use tokio::net::TcpListener;
use traq::apis::configuration::Configuration;
use traq_bot_http::{Event, RequestParser};

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
            } else if payload.message.plain_text.contains("aki") {
                let res = akinator::Akinator::new(akinator::AkinatorGameTheme::Character)
                    .start()
                    .await;
                match res {
                    Ok(akinator::AkinatorState::Question(question)) => question.question,
                    Ok(akinator::AkinatorState::Guess(guess)) => {
                        format!("{} ですか", guess.name)
                    }
                    Err(e) => {
                        eprintln!("{e}");
                        return StatusCode::INTERNAL_SERVER_ERROR;
                    }
                }
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
