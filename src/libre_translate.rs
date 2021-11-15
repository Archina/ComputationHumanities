use serde_derive::{Serialize, Deserialize};

use crate::data::Language;

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload{
    pub q: String,
    pub source: Language,
    pub target: Language
}

#[derive(Deserialize)]
struct LibreTranslateResponse{
    #[serde(rename="translatedText")]
    translated_text: String
}

pub async fn request(body: &Payload) -> Option<String> {
    let client = reqwest::Client::new();
    let res = client.post("https://libretranslate.de/translate")
        .body(serde_json::to_string(body).unwrap())
        .header("Content-Type", "application/json")
        .send()
        .await.unwrap();
    res.text().await.ok().and_then(|r| serde_json::from_str::<LibreTranslateResponse>(&r).ok().map(|r| r.translated_text))
}