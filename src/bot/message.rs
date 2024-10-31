use http::StatusCode;
use traq_bot_http::payloads::{
    BotMessageStampsUpdatedPayload, DirectMessageCreatedPayload, MessageCreatedPayload,
};

use super::game;

pub async fn on_message_created(
    state: crate::AppState,
    payload: MessageCreatedPayload,
) -> StatusCode {
    let message = payload.message;

    if message.plain_text.contains("aki") {
        game::aki::game_theme_select(&state.app, &message.channel_id).await;
    }

    StatusCode::NO_CONTENT
}

pub async fn on_bot_message_stamps_updated(
    state: crate::AppState,
    payload: BotMessageStampsUpdatedPayload,
) -> StatusCode {
    let message_id = payload.message_id;

    StatusCode::NO_CONTENT
}

pub async fn on_direct_message_created(
    state: crate::AppState,
    payload: DirectMessageCreatedPayload,
) -> StatusCode {
    let message = payload.message;

    StatusCode::NO_CONTENT
}
