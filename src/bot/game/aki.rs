use akinator::{Akinator, AkinatorGameTheme, AkinatorProposition, AkinatorQuestion};
use axum::async_trait;
use http::StatusCode;
use traq::apis::message_api;

use crate::{App, AppState};

use super::Game;

//:viine02:
const CHARA_STAMP_ID: &str = "a96defd6-49cb-4f44-adaa-a4bf511a90fd";

//:cat:
const ANIMAL_STAMP_ID: &str = "964c82e8-e1ec-40d2-9a22-82b2c48794f2";

//:one:
const YES_STAMP_ID: &str = "9f0be841-fbfa-4abf-871e-c1c72627e691";

//:two:
const NO_STAMP_ID: &str = "350c45b4-a048-4f62-bf2b-e98f4edef40c";

//:three:
const DONT_KNOW_STAMP_ID: &str = "ea0e7725-5b86-456b-b34a-060035153be2";

//:four:
const PROBABLY_STAMP_ID: &str = "1463cc12-9758-478f-b968-e031a912d426";

//:five:
const PROBABLY_NOT_STAMP_ID: &str = "2d04f8d3-b2db-4e53-b11c-111350c7b70d";

//:arrow_backward:
const BACK_STAMP_ID: &str = "118d80c7-6766-44d1-b3fc-945e94108350";

pub enum AkinatorGameState {
    Init,
    ThemeSelect,
    Question,
    Guess,
    End,
}

#[derive(Clone)]
pub struct AkinatorGameMessage {
    message_id: String,
    channel_id: String,
}
pub struct AkinatorGame {
    game: Option<Akinator>,
    last_message: Option<AkinatorGameMessage>,
    state: AkinatorGameState,
}

impl AkinatorGame {
    pub fn new() -> Self {
        Self {
            game: None,
            last_message: None,
            state: AkinatorGameState::Init,
        }
    }
}

#[async_trait]
impl Game for AkinatorGame {
    async fn on_message_created(
        &mut self,
        app: &AppState,
        payload: &traq_bot_http::payloads::MessageCreatedPayload,
    ) -> StatusCode {
        let message = &payload.message.plain_text;
        if self.game.is_some() || !message.contains("akinator") {
            return StatusCode::NO_CONTENT;
        }
        let res = self
            .game_theme_select(&app.app, &payload.message.channel_id)
            .await;
        if res.is_success() {
            self.last_message = Some(AkinatorGameMessage {
                message_id: payload.message.id.to_string(),
                channel_id: payload.message.channel_id.to_string(),
            });
            self.state = AkinatorGameState::ThemeSelect;
        }

        res
    }

    async fn on_bot_message_stamps_updated(
        &mut self,
        app: &AppState,
        payload: &traq_bot_http::payloads::BotMessageStampsUpdatedPayload,
    ) -> StatusCode {
        let message_id = &payload.message_id;
        let last_message = match self.last_message.as_ref() {
            Some(last_message) => last_message.clone(),
            None => {
                return StatusCode::NO_CONTENT;
            }
        };
        if &last_message.message_id != message_id {
            return StatusCode::NO_CONTENT;
        }

        let stamp = payload.stamps.iter().find(|s| s.count >= 2);
        if let Some(stamp) = stamp {
            match self.state {
                AkinatorGameState::Init => StatusCode::NO_CONTENT,
                AkinatorGameState::ThemeSelect => {
                    self.start_game(&app.app, &last_message.channel_id, &stamp.stamp_id)
                        .await
                }
                AkinatorGameState::Question => {
                    self.answer_question(&app.app, &last_message.channel_id, &stamp.stamp_id)
                        .await
                }
                AkinatorGameState::Guess => {
                    self.answer_guess(&app.app, &last_message.channel_id, &stamp.stamp_id)
                        .await
                }
                AkinatorGameState::End => StatusCode::NO_CONTENT,
            }
        } else {
            StatusCode::NO_CONTENT
        }
    }

    async fn on_direct_message_created(
        &mut self,
        app: &AppState,
        payload: &traq_bot_http::payloads::DirectMessageCreatedPayload,
    ) -> StatusCode {
        let message = &payload.message.plain_text;
        if self.game.is_some() || !message.contains("akinator") {
            return StatusCode::NO_CONTENT;
        }
        let res = self
            .game_theme_select(&app.app, &payload.message.channel_id)
            .await;
        if res.is_success() {
            self.last_message = Some(AkinatorGameMessage {
                message_id: payload.message.id.to_string(),
                channel_id: payload.message.channel_id.to_string(),
            });
            self.state = AkinatorGameState::ThemeSelect;
        }

        res
    }

    async fn destroy(&mut self) -> StatusCode {
        StatusCode::NO_CONTENT
    }
}
impl AkinatorGame {
    async fn game_theme_select(&mut self, app: &App, channel_id: &str) -> StatusCode {
        let req_message = "
        ## Akinator\n
        ### やあ、私はアキネイターです:doya-nya.ex-large:\n
        有名な人物やキャラクターを思い浮かべて。魔人が誰でも当ててみせよう。魔人は何でもお見通しさ\n\n
        #### ゲームのテーマを選んでください。\n
        #### :one.ex-large: : キャラクター\n
        #### :two.ex-large: : 動物\n
        "
    .to_string();

        let request = traq::models::PostMessageRequest {
            content: req_message,
            embed: Some(true),
        };
        let res = message_api::post_message(&app.client_config, channel_id, Some(request)).await;

        let message = match res {
            Ok(message) => message,
            Err(e) => {
                eprintln!("Error: {e}");
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
        };

        self.post_stamps(
            app,
            &message.id.to_string(),
            vec![CHARA_STAMP_ID, ANIMAL_STAMP_ID],
        )
        .await
    }

    async fn start_game(&mut self, app: &App, channel_id: &str, stamp_id: &str) -> StatusCode {
        let theme = match stamp_id {
            CHARA_STAMP_ID => AkinatorGameTheme::Character,
            ANIMAL_STAMP_ID => AkinatorGameTheme::Animal,
            _ => {
                eprintln!("Invalid stamp_id: {stamp_id}");
                return StatusCode::BAD_REQUEST;
            }
        };

        let akinator = Akinator::new(theme);
        self.game = Some(akinator);

        let res = self.game.as_mut().unwrap().start().await;

        match res {
            Ok(akinator::AkinatorState::Question(question)) => {
                self.state = AkinatorGameState::Question;
                self.post_question(app, question, channel_id).await
            }
            Ok(akinator::AkinatorState::Guess(guess)) => {
                self.state = AkinatorGameState::Guess;
                self.post_guess(app, guess, channel_id).await
            }
            Err(e) => {
                tracing::error!("Error: {e}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    async fn answer_question(&mut self, app: &App, channel_id: &str, stamp_id: &str) -> StatusCode {
        let akinator = match self.game.as_mut() {
            Some(akinator) => akinator,
            None => {
                tracing::error!("Game is not started");
                return StatusCode::BAD_REQUEST;
            }
        };
        let res = match stamp_id {
            YES_STAMP_ID => akinator.answer(AkinatorProposition::Yes).await,
            NO_STAMP_ID => akinator.answer(AkinatorProposition::No).await,
            DONT_KNOW_STAMP_ID => akinator.answer(AkinatorProposition::DontKnow).await,
            PROBABLY_STAMP_ID => akinator.answer(AkinatorProposition::Probably).await,
            PROBABLY_NOT_STAMP_ID => akinator.answer(AkinatorProposition::ProbablyNot).await,
            BACK_STAMP_ID => akinator.back().await,
            _ => {
                eprintln!("Invalid stamp_id: {stamp_id}");
                return StatusCode::BAD_REQUEST;
            }
        };
        match res {
            Ok(akinator::AkinatorState::Question(question)) => {
                self.state = AkinatorGameState::Question;
                self.post_question(app, question, channel_id).await
            }
            Ok(akinator::AkinatorState::Guess(guess)) => {
                self.state = AkinatorGameState::Guess;
                self.post_guess(app, guess, channel_id).await
            }
            Err(e) => {
                tracing::error!("Error: {e}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    async fn answer_guess(&mut self, app: &App, channel_id: &str, stamp_id: &str) -> StatusCode {
        let is_correct = match stamp_id {
            YES_STAMP_ID => true,
            NO_STAMP_ID => false,
            _ => {
                eprintln!("Invalid stamp_id: {stamp_id}");
                return StatusCode::BAD_REQUEST;
            }
        };

        let akinator = match self.game.as_mut() {
            Some(akinator) => akinator,
            None => {
                tracing::error!("Game is not started");
                return StatusCode::BAD_REQUEST;
            }
        };

        if is_correct {
            self.post_end_game(app, channel_id).await;
            self.state = AkinatorGameState::End;
            StatusCode::NO_CONTENT
        } else {
            let res = akinator.exclude().await;
            match res {
                Ok(akinator::AkinatorState::Question(question)) => {
                    self.state = AkinatorGameState::Question;
                    self.post_question(app, question, channel_id).await
                }
                Ok(akinator::AkinatorState::Guess(guess)) => {
                    self.state = AkinatorGameState::Guess;
                    self.post_guess(app, guess, channel_id).await
                }
                Err(e) => {
                    tracing::error!("Error: {e}");
                    StatusCode::INTERNAL_SERVER_ERROR
                }
            }
        }
    }

    async fn post_question(
        &mut self,
        app: &App,
        question: AkinatorQuestion,
        channel_id: &str,
    ) -> StatusCode {
        let req_message = format!(
            "## Akinator\n\n
        進捗度: {}\n\n
        #### 質問 {}: {}\n\n
        #### :one.ex-large: : はい\n
        #### :two.ex-large: : いいえ\n
        #### :three.ex-large: : わからない\n
        #### :four.ex-large: : たぶんそう、部分的にそう\n
        #### :five.ex-large: : たぶん違う、部分的にちがう\n
        #### :arrow_backward.ex-large: : 戻る\n",
            question.progression, question.step, question.question
        );
        let request = traq::models::PostMessageRequest {
            content: req_message,
            embed: Some(true),
        };
        let res = message_api::post_message(&app.client_config, channel_id, Some(request)).await;
        if let Err(e) = res {
            tracing::error!("Error: {e}");
            return StatusCode::INTERNAL_SERVER_ERROR;
        }

        self.post_stamps(
            app,
            channel_id,
            vec![
                YES_STAMP_ID,
                NO_STAMP_ID,
                DONT_KNOW_STAMP_ID,
                PROBABLY_STAMP_ID,
                PROBABLY_NOT_STAMP_ID,
            ],
        )
        .await
    }

    async fn post_guess(
        &mut self,
        app: &App,
        guess: akinator::AkinatorGuess,
        channel_id: &str,
    ) -> StatusCode {
        let req_message = format!(
            "## Akinator\n\n
        あなたが思い浮かべたキャラクターは {} ですか？\n\n
        
        #### :one.ex-large: : はい\n
        #### :two.ex-large: : いいえ\n",
            guess.name
        );
        let request = traq::models::PostMessageRequest {
            content: req_message,
            embed: Some(true),
        };
        let res = message_api::post_message(&app.client_config, channel_id, Some(request)).await;
        match res {
            Err(e) => {
                tracing::error!("Error: {e}");
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
            Ok(res) => {
                let res_message: AkinatorGameMessage = AkinatorGameMessage {
                    message_id: res.id.to_string(),
                    channel_id: channel_id.to_string(),
                };
                self.last_message = Some(res_message);
            }
        }

        self.post_stamps(app, channel_id, vec![YES_STAMP_ID, NO_STAMP_ID])
            .await
    }

    async fn post_end_game(&mut self, app: &App, channel_id: &str) -> StatusCode {
        let req_message = "## Akinator\n\nゲームを終了しました。".to_string();
        let request = traq::models::PostMessageRequest {
            content: req_message,
            embed: Some(true),
        };
        let res = message_api::post_message(&app.client_config, channel_id, Some(request)).await;
        match res {
            Err(e) => {
                tracing::error!("Error: {e}");
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
            Ok(_) => {
                self.game = None;
                self.last_message = None;
            }
        }

        StatusCode::NO_CONTENT
    }

    async fn post_stamps(
        &mut self,
        app: &App,
        channel_id: &str,
        stamp_ids: Vec<&str>,
    ) -> StatusCode {
        for stamp_id in stamp_ids {
            if let Err(e) = traq::apis::stamp_api::add_message_stamp(
                &app.client_config,
                channel_id,
                stamp_id,
                None,
            )
            .await
            {
                eprintln!("Error: {e}");
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
        }

        StatusCode::NO_CONTENT
    }
}
