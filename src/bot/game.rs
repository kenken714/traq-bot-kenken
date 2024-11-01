use async_session::chrono::{self, Duration, Utc};
use axum::async_trait;
use chrono::prelude::*;
use http::StatusCode;
use traq_bot_http::payloads;

use crate::AppState;

pub mod aki;

#[async_trait]
pub trait Game: Send + Sync {
    async fn on_message_created(
        &mut self,
        app: &AppState,
        payload: &payloads::MessageCreatedPayload,
    ) -> StatusCode;
    async fn on_bot_message_stamps_updated(
        &mut self,
        app: &AppState,
        payload: &payloads::BotMessageStampsUpdatedPayload,
    ) -> StatusCode;
    async fn on_direct_message_created(
        &mut self,
        app: &AppState,
        payload: &payloads::DirectMessageCreatedPayload,
    ) -> StatusCode;
    async fn destroy(&mut self) -> StatusCode;
}

pub struct GameSession {
    game: Box<dyn Game>,
    expiration: DateTime<Utc>,
}

pub struct GameSessionManager {
    sessions: Vec<GameSession>,
}

impl GameSessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Vec::new(),
        }
    }

    pub fn set_game(&mut self, game: Box<dyn Game>) {
        let expiration = Utc::now() + Duration::minutes(3);
        self.sessions.push(GameSession { game, expiration });
    }

    //FIXME: 複数受理するか１つだけ受理するかは要検討
    pub async fn on_message_created(
        &mut self,
        app: &AppState,
        payload: &payloads::MessageCreatedPayload,
    ) -> StatusCode {
        self.delete_expired_sessions().await;
        for session in &mut self.sessions {
            session.game.on_message_created(app, payload).await;
        }

        StatusCode::NO_CONTENT
    }

    pub async fn on_direct_message_created(
        &mut self,
        app: &AppState,
        payload: &payloads::DirectMessageCreatedPayload,
    ) -> StatusCode {
        self.delete_expired_sessions().await;
        for session in &mut self.sessions {
            session.game.on_direct_message_created(app, payload).await;
        }

        StatusCode::NO_CONTENT
    }

    pub async fn on_bot_message_stamps_updated(
        &mut self,
        app: &AppState,
        payload: &payloads::BotMessageStampsUpdatedPayload,
    ) -> StatusCode {
        self.delete_expired_sessions().await;
        for session in &mut self.sessions {
            session
                .game
                .on_bot_message_stamps_updated(app, payload)
                .await;
        }

        StatusCode::NO_CONTENT
    }

    async fn delete_expired_sessions(&mut self) {
        let expired_indices: Vec<usize> = self
            .sessions
            .iter()
            .enumerate()
            .filter_map(|(i, session)| {
                if session.expiration < Utc::now() {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();

        for &i in expired_indices.iter().rev() {
            let mut session = self.sessions.remove(i);
            session.game.destroy().await;
        }
    }
}
