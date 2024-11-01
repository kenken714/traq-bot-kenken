use serde::{Deserialize, Serialize};
use serde_repr::Serialize_repr;

// own crate
use error::AkinatorError;

mod cli;
pub mod error;
mod utils;

enum Language {
    English,
    Japanese,
    Chinese,
}

impl Default for Language {
    fn default() -> Self {
        Self::Japanese
    }
}

impl Language {
    fn to_lang_code(&self) -> &str {
        match self {
            Self::English => "en",
            Self::Japanese => "jp",
            Self::Chinese => "cn",
        }
    }
}

#[derive(Serialize_repr, Clone, Copy)]
#[repr(u8)]
pub enum AkinatorProposition {
    Yes = 0,
    No = 1,
    DontKnow = 2,
    Probably = 3,
    ProbablyNot = 4,
}

#[derive(Clone, Copy)]
pub enum AkinatorGameTheme {
    Character,
    Animal,
}

impl AkinatorGameTheme {
    fn to_sid(self) -> i32 {
        match self {
            Self::Character => 1,
            Self::Animal => 14,
        }
    }
}

pub enum AkinatorState {
    Question(AkinatorQuestion),
    Guess(AkinatorGuess),
}

pub struct AkinatorQuestion {
    pub question: String,
    pub step: i32,
    pub progression: f32,
    pub akitude: reqwest::Url,
}

pub struct AkinatorGuess {
    pub name: String,
    pub description: String,
    pub image: reqwest::Url,
}

pub struct Akinator {
    params: AkinatorParams,
    session: Option<AkinatorSession>,
}

struct AkinatorParams {
    step: i32,
    progression: f32,
    step_last_proposition: Option<i32>,
    url: reqwest::Url,
    language: Language,
    child_mode: bool,
    game_theme: AkinatorGameTheme,
    miss_count: i32,
}

impl AkinatorParams {
    fn new(game_theme: AkinatorGameTheme) -> Self {
        Self {
            url: reqwest::Url::parse(&format!(
                "https://{}.akinator.com",
                Language::default().to_lang_code()
            ))
            .unwrap(),
            language: Language::default(),
            step: 0,
            progression: 0.0,
            step_last_proposition: None,
            child_mode: false,
            game_theme,
            miss_count: 0,
        }
    }
}
struct AkinatorSession {
    session: String,
    signature: String,
}

#[derive(Serialize)]
struct AkinatorStart {
    sid: i32,
    cm: bool,
}

impl AkinatorStart {
    fn new(game_theme: AkinatorGameTheme, child_mode: bool) -> Self {
        let sid = game_theme.to_sid();
        Self {
            sid,
            cm: child_mode,
        }
    }
}

#[derive(Serialize)]
struct AkinatorRequest {
    sid: i32,
    cm: bool,
    step: i32,
    progression: f32,
    step_last_proposition: Option<i32>,
    answer: Option<AkinatorProposition>,
    session: String,
    signature: String,
}

// FIXME: きたない
#[derive(Deserialize)]
struct AkinatorResponse {
    completion: String,
    //only present when akinator asks a question
    question: Option<String>,
    step: Option<String>,
    progression: Option<String>,
    akitude: String,
    //only present when akinator guesses
    id_base_proposition: Option<String>,
    name_proposition: Option<String>,
    description_proposition: Option<String>,
    photo: Option<String>,
}

impl Akinator {
    pub fn new(game_theme: AkinatorGameTheme) -> Self {
        Self {
            params: AkinatorParams::new(game_theme),
            session: None,
        }
    }

    pub async fn start(&mut self) -> Result<AkinatorState, AkinatorError> {
        let start_request = AkinatorStart::new(self.params.game_theme, self.params.child_mode);
        let response = self.send_request("game", &start_request).await?;

        let body = response
            .text()
            .await
            .map_err(|_| AkinatorError::InvalidResponse)?;

        let session = utils::session_from_body(&body).await?;
        self.session = Some(session);

        let question = utils::get_question_from_body(&body).await?;

        Ok(AkinatorState::Question(AkinatorQuestion {
            question,
            step: 0,
            progression: 0.0,
            akitude: reqwest::Url::parse(&format!(
                "{}/assets/img/akitudes_670x1096/defy.png",
                self.params.url
            ))
            .map_err(|_| AkinatorError::UnexpectedError)?,
        }))
    }

    pub async fn answer(
        &mut self,
        answer: AkinatorProposition,
    ) -> Result<AkinatorState, AkinatorError> {
        let session = self
            .session
            .as_ref()
            .ok_or(AkinatorError::SessionNotFound)?;

        let params = &self.params;

        let answer_request = AkinatorRequest {
            sid: params.game_theme.to_sid(),
            cm: params.child_mode,
            step: params.step,
            progression: params.progression,
            step_last_proposition: params.step_last_proposition,
            answer: Some(answer),
            session: session.session.clone(),
            signature: session.signature.clone(),
        };

        let res = self.send_ingame_request("answer", &answer_request).await?;
        match res.completion.as_str() {
            "OK" => self.update_state(res),
            // アキネーターの解答回数が上限に達した
            "SOUNDLIKE" => Err(AkinatorError::QuestionLimitExceeded),
            _ => Err(AkinatorError::InvalidResponse),
        }
    }

    pub async fn exclude(&mut self) -> Result<AkinatorState, AkinatorError> {
        let session = self
            .session
            .as_ref()
            .ok_or(AkinatorError::SessionNotFound)?;

        let params = &self.params;

        let exclude_request = AkinatorRequest {
            sid: params.game_theme.to_sid(),
            cm: params.child_mode,
            step: params.step,
            progression: params.progression,
            step_last_proposition: params.step_last_proposition,
            answer: None,
            session: session.session.clone(),
            signature: session.signature.clone(),
        };

        let res = self
            .send_ingame_request("exclude", &exclude_request)
            .await?;

        self.params.miss_count += 1;

        self.update_state(res)
    }

    pub async fn back(&mut self) -> Result<AkinatorState, AkinatorError> {
        let session = self
            .session
            .as_ref()
            .ok_or(AkinatorError::SessionNotFound)?;

        let params = &self.params;

        if params.step == 0 {
            return Err(AkinatorError::CannotBackAnyMore);
        }

        let back_request = AkinatorRequest {
            sid: params.game_theme.to_sid(),
            cm: params.child_mode,
            step: params.step,
            progression: params.progression,
            step_last_proposition: None,
            answer: None,
            session: session.session.clone(),
            signature: session.signature.clone(),
        };

        let res = self
            .send_ingame_request("cancel_answer", &back_request)
            .await?;
        self.update_state(res)
    }

    fn update_state(&mut self, response: AkinatorResponse) -> Result<AkinatorState, AkinatorError> {
        let is_guessed = response.id_base_proposition.is_some();
        // 解答された場合
        if is_guessed {
            self.params.step_last_proposition = Some(self.params.step);

            Ok(AkinatorState::Guess(AkinatorGuess {
                name: response
                    .name_proposition
                    .ok_or(AkinatorError::InvalidResponse)?,
                description: response
                    .description_proposition
                    .ok_or(AkinatorError::InvalidResponse)?,
                image: reqwest::Url::parse(&response.photo.ok_or(AkinatorError::InvalidResponse)?)
                    .map_err(|_| AkinatorError::InvalidResponse)?,
            }))
        }
        // 質問された場合
        else {
            let step: i32 = response
                .step
                .ok_or(AkinatorError::InvalidResponse)?
                .parse()
                .map_err(|_| AkinatorError::InvalidResponse)?;
            let progression: f32 = response
                .progression
                .ok_or(AkinatorError::InvalidResponse)?
                .parse()
                .map_err(|_| AkinatorError::InvalidResponse)?;

            self.params.step = step;
            self.params.progression = progression;

            Ok(AkinatorState::Question(AkinatorQuestion {
                question: response.question.ok_or(AkinatorError::InvalidResponse)?,
                step,
                progression,
                akitude: reqwest::Url::parse(&format!(
                    "{}/assets/img/akitudes_670x1096/{}",
                    self.params.url, response.akitude
                ))
                .map_err(|_| AkinatorError::InvalidResponse)?,
            }))
        }
    }
}
