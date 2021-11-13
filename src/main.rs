extern crate serde_derive;

use std::{fs::File, io::BufReader, path::Path};

use reqwest::Response;
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
enum Language{
    #[serde(rename = "ar")]
    Arabic,
    #[serde(rename = "de")]
    German,
    #[serde(rename = "en")]
    English,
    #[serde(rename = "es")]
    Spanish,
    #[serde(rename = "fi")]
    Finnish,
    #[serde(rename = "fr")]
    French,
    #[serde(rename = "ga")]
    Irish,
    #[serde(rename = "hi")]
    Hindi,
    #[serde(rename = "hu")]
    Hungarian,
    #[serde(rename = "id")]
    Indonesian,
    #[serde(rename = "it")]
    Italian,
    #[serde(rename = "ja")]
    Japanese,
    #[serde(rename = "ko")]
    Korean,
    #[serde(rename = "nl")]
    Dutch,
    #[serde(rename = "pl")]
    Polish,
    #[serde(rename = "pt")]
    Portuguese,
    #[serde(rename = "ru")]
    Russian,
    #[serde(rename = "sv")]
    Swedish,
    #[serde(rename = "tr")]
    Turkish,
    #[serde(rename = "uk")]
    Ukranian,
    #[serde(rename = "vi")]
    Vietnamese,
    #[serde(rename = "zh")]
    Chinese
}

#[derive(Serialize, Deserialize, Debug)]
struct Payload{
    q: String,
    source: Language,
    target: Language
}

#[derive(Serialize, Deserialize, Debug)]
struct Sentence{
    id: u32,
    lang: Language,
    text: String
}

#[derive(Serialize, Deserialize, Debug)]
struct DataSet{
    sentences: Vec<Sentence>,
}


fn read_user_from_file<P: AsRef<Path>>(path: P) -> Option<DataSet> {
    File::open(path)
        .ok()
        .map(|file| BufReader::new(file))
        .and_then(|r| serde_json::from_reader(r).ok())
}

#[derive(Deserialize)]
struct LibreTranslateResponse{
    translatedText: String
}

async fn request(body: &Payload) -> Option<LibreTranslateResponse> {
    let client = reqwest::Client::new();
    let res = client.post("https://libretranslate.de/translate")
        .body(serde_json::to_string(body).unwrap())
        .header("Content-Type", "application/json")
        .send()
        .await.unwrap();
    res.text().await.ok().and_then(|r| serde_json::from_str(&r).ok())
}

#[async_std::main]
async fn main() {
    println!("Hello, world!");

    let j = b"
    {
    \"q\": \"0xF9BA143B95FF6D82\",
    \"source\": \"es\",
    \"target\": \"zh\"
    }";

    let test: Payload = serde_json::from_slice(j).unwrap();

    // println!("{:?}", test);

    let dataset = format!("data/dataset-{}.json", serde_json::to_string(&Language::Chinese).unwrap().replace("\"", ""));
    let content = read_user_from_file(dataset).unwrap();
    for sentence in content.sentences {
        println!("{}", sentence.text);
    }

    // let query: String = "I am just a little test! Please be nice to me.".into();
    // let result = round_trip(
    //     &Language::English,
    //     &Language::German,
    //     &query
    // ).await;

    // println!("{}\n{:?}", query, result);
}

async fn translate(from: &Language, to: &Language, text: &String) -> Option<String> {
    let response = request(&Payload{
        q: text.clone(),
        source: from.clone(),
        target: to.clone()
    }).await;
    response.map(|r| r.translatedText)
}

async fn round_trip(origin: &Language, target: &Language, text: &String) -> (String, String) {
    let s_1 = translate(origin, target, text).await;
    let s_2 = translate(target, origin, s_1.as_ref().unwrap()).await;
    (s_1.unwrap(), s_2.unwrap())
}
