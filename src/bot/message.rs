use http::StatusCode;
use traq::{apis::message_api::post_message, models::PostMessageRequest};
use traq_bot_http::payloads::{
    BotMessageStampsUpdatedPayload, DirectMessageCreatedPayload, MessageCreatedPayload,
};

use super::game;

#[tracing::instrument]
pub async fn on_message_created(
    state: crate::AppState,
    payload: MessageCreatedPayload,
) -> StatusCode {
    let message = &payload.message;
    tracing::info!(
        "Channel ID: {} で {} がメッセージを送信",
        message.channel_id,
        message.user.id
    );

    //FIXME: ここで依存性注入するのはやばい
    if message.plain_text.contains("akinator") {
        let game = game::aki::AkinatorGame::new();
        state.game_sessions.lock().await.set_game(Box::new(game));
    }

    if message.plain_text.contains("おいすー") {
        let request = PostMessageRequest {
            content: format!("@{} おいすー！！！", message.user.id),
            embed: Some(true),
        };
        let res = post_message(&state.app.client_config, &message.channel_id, Some(request)).await;
        tracing::info!("{:?}", res);
    }

    state
        .game_sessions
        .lock()
        .await
        .on_message_created(&state, &payload)
        .await;

    StatusCode::NO_CONTENT
}

#[tracing::instrument]
pub async fn on_bot_message_stamps_updated(
    state: crate::AppState,
    payload: BotMessageStampsUpdatedPayload,
) -> StatusCode {
    tracing::info!("Botのメッセージスタンプが更新されました");
    state
        .game_sessions
        .lock()
        .await
        .on_bot_message_stamps_updated(&state, &payload)
        .await;
    StatusCode::NO_CONTENT
}

#[tracing::instrument]
pub async fn on_direct_message_created(
    state: crate::AppState,
    payload: DirectMessageCreatedPayload,
) -> StatusCode {
    let message = &payload.message;
    tracing::info!(
        "Channel ID: {} で {} がメッセージを送信",
        message.channel_id,
        message.user.id
    );
    if message.plain_text.contains("おいすー") {
        let request = PostMessageRequest {
            content: format!("@{} おいすー！！！", message.user.id),
            embed: Some(true),
        };
        let res = post_message(&state.app.client_config, &message.channel_id, Some(request)).await;
        tracing::info!("{:?}", res);
    }

    state
        .game_sessions
        .lock()
        .await
        .on_direct_message_created(&state, &payload)
        .await;

    StatusCode::NO_CONTENT
}
