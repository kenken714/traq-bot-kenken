use std::sync::Arc;
use tokio::sync::Mutex;

use axum::{routing::post, Router};

use crate::bot::game::GameSessionManager;
use crate::{repository::Repository, App, AppState};

use crate::bot;

pub async fn make_router(
    app: App,
    infra: Arc<Repository>,
    game_sessions: Arc<Mutex<GameSessionManager>>,
) -> Router {
    let state = AppState {
        app,
        infra,
        game_sessions,
    };
    Router::new()
        .route("/bot", post(bot::handle_event))
        .with_state(state)
}
