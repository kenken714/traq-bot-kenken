use axum::{body::Bytes, extract::State};
use http::{HeaderMap, StatusCode};
use tracing::error;
use traq_bot_http::Event;

use crate::AppState;

pub(crate) mod game;
mod message;
mod util;

pub async fn handle_event(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> StatusCode {
    let event = match state.app.request_parser.parse(&headers, &body) {
        Ok(event) => event,
        Err(e) => {
            error!("Failed to parse request: {}", e);
            return StatusCode::BAD_REQUEST;
        }
    };
    match event {
        Event::MessageCreated(payload) => message::on_message_created(state, payload).await,
        Event::BotMessageStampsUpdated(payload) => {
            message::on_bot_message_stamps_updated(state, payload).await
        }
        Event::DirectMessageCreated(payload) => {
            message::on_direct_message_created(state, payload).await
        }
        _ => StatusCode::NO_CONTENT,
    }
}
