use http::uri;

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

enum AkinatorProposition {
    Yes,
    Probably,
    ProbablyNot,
    No,
    DontKnow,
}

struct Akinator {
    question: String,
    step: i32,
    progression: f32,
    step_last_proposition: Option<AkinatorProposition>,
    guessed: bool,
    guess_name: Option<String>,
    guess_description: Option<String>,
    guess_image: Option<uri::Uri>,
    uri: uri::Uri,
    akitude: uri::PathAndQuery,
    language: Language,
    child_mode: bool,
}

impl Akinator {
    fn new() -> Self {
        Self {
            uri: uri::Uri::try_from(
                format!(
                    "https://{}.akinator.com",
                    Language::default().to_lang_code()
                )
                .as_str(),
            )
            .unwrap(),
            akitude: uri::PathAndQuery::from_static("defi.png"),
            language: Language::default(),
            question: String::new(),
            step: 0,
            progression: 0.0,
            step_last_proposition: None,
            guessed: false,
            guess_name: None,
            guess_description: None,
            guess_image: None,
            child_mode: false,
        }
    }
}

async fn akinator_handler() -> Result<HttpResponse, Error> {
    let akinator = Akinator::new();
    let response = akinator.start().await;
    Ok(HttpResponse::Ok().json(response))
}
