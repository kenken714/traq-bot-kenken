use repository::Repository;
use std::{env, net::SocketAddr, sync::Arc};
use tokio::{net::TcpListener, sync::Mutex};
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;
use traq::apis::configuration::Configuration;
use traq_bot_http::RequestParser;

use crate::bot::game::GameSessionManager;

mod bot;
mod repository;
mod router;

#[derive(Clone)]
struct App {
    request_parser: RequestParser,
    client_config: Configuration,
}

#[derive(Clone)]
struct AppState {
    app: App,
    infra: Arc<Repository>,
    game_sessions: Arc<Mutex<GameSessionManager>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or("info".into()))
        .init();

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

    let game_sessions = GameSessionManager::new();

    let router = router::make_router(
        app.clone(),
        Arc::new(infra),
        Arc::new(Mutex::new(game_sessions)),
    )
    .await
    .layer(TraceLayer::new_for_http());
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = TcpListener::bind(&addr).await.unwrap();

    tracing::info!("Listening on {}", addr);
    axum::serve(listener, router).await?;

    Ok(())
}
