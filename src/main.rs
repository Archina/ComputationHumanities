extern crate serde_derive;

use data::Language;
use libre_translate::{Payload, request};

use crate::data::read_user_from_file;

mod data;
mod libre_translate;
mod n_gram;

#[async_std::main]
async fn main() {

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

pub async fn translate(from: &Language, to: &Language, text: &String) -> Option<String> {
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
