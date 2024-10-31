use akinator::Akinator;
use async_session::chrono::{self, Duration, Utc};
use chrono::prelude::*;

pub mod aki;

pub enum Game {
    Akinator(Akinator),
}

impl Game {
    pub async fn start(&mut self) {
        match self {
            Game::Akinator(akinator) => {
                akinator.start().await.unwrap();
            }
        }
    }
}
pub struct GameSession {
    game: Game,
    expiration: DateTime<Utc>,
    last_message_id: Option<String>,
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

    pub fn set_game(&mut self, game: Game, last_message_id: Option<String>) {
        let expiration = Utc::now() + Duration::minutes(3);
        self.sessions.push(GameSession {
            game,
            expiration,
            last_message_id,
        });
    }

    pub fn get_game_from_message_id(&mut self, message_id: &str) -> Option<&Game> {
        self.delete_expired_sessions();
        self.sessions
            .iter()
            .find(|session| session.last_message_id == Some(message_id.to_string()))
            .map(|session| &session.game)
    }

    fn delete_expired_sessions(&mut self) {
        self.sessions
            .retain(|session| session.expiration > Utc::now());
    }
}
