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

async fn request(body: &Payload) -> Option<String> {
    let client = reqwest::Client::new();
    let res = client.post("https://libretranslate.de/translate")
        .body(serde_json::to_string(body).unwrap())
        .header("Content-Type", "application/json")
        .send()
        .await.unwrap();
    res.text().await.ok().and_then(|r| serde_json::from_str::<LibreTranslateResponse>(&r).ok().map(|r| r.translated_text))
}

async fn translate(from: &Language, to: &Language, text: &String) -> Option<String> {
    request(&Payload{
        q: text.clone(),
        source: from.clone(),
        target: to.clone()
    }).await
}

pub async fn round_trip(origin: &Language, target: &Language, text: &String) -> (String, String) {
    let s_1 = translate(origin, target, text).await;
    let s_2 = translate(target, origin, s_1.as_ref().unwrap()).await;
    (s_1.unwrap(), s_2.unwrap())
}