use std::sync::Arc;

use axum::{routing::post, Router};
use traq_bot_http::RequestParser;

use crate::{repository::Repository, App, AppState};

use crate::bot;

pub async fn make_router(verification_token: &str, app: App, infra: Arc<Repository>) -> Router {
    let parser = RequestParser::new(verification_token);
    let state = AppState { app, infra, parser };
    Router::new()
        .route("/bot", post(bot::handle_event))
        .with_state(state)
}
