extern crate serde_derive;

use csv::Writer;
use data::Language;
use libre_translate::{round_trip};
use serde_derive::Serialize;

use crate::{data::read_data_set_from_file, n_gram::{bleu_metric, fragments}};

mod data;
mod libre_translate;
mod n_gram;

#[async_std::main]
async fn main() {

    // let dataset = format!("data/dataset-{}.json", serde_json::to_string(&Language::Chinese).unwrap().replace("\"", ""));
    // let content = read_user_from_file(dataset).unwrap();
    // for sentence in content.sentences {
    //     println!("{}", sentence.text);
    // }
    
    let results = translate_set(&Language::Chinese, &Language::German).await;

    let buffer = std::fs::File::create("output/Test.csv").unwrap();
    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .quote_style(csv::QuoteStyle::NonNumeric).from_writer(buffer);
    // let mut wtr = Writer::from_writer(buffer);
    for entry in results{
        wtr.serialize(entry);
    }

    // let query: String = "I am just a little test! Please be nice to me.".into();
    // let result = round_trip(
    //     &Language::English,
    //     &Language::German,
    //     &query
    // ).await;

    // println!("{}\n{:?}", query, result);
}

#[derive(Serialize)]
struct Record{
    from: Language,
    original: String,
    reverse_translation: String,
    to: Language,
    translation: String,
    bleu: f32
}

async fn translate_set(from: &Language, to: &Language) -> Vec<Record> {
    let dataset_file_name = format!("data/dataset-{}.json", serde_json::to_string(&from).unwrap().replace("\"", ""));
    let content = read_data_set_from_file(dataset_file_name).unwrap();
    let mut records = vec![];
    // if let Some(sentence) = content.sentences.first() {
    // }
    for sentence in content.sentences {
        let (translation, reverse_translation) = round_trip(from, to, &sentence.text).await;
        println!("{}\n{}\n{}", sentence.text, translation, reverse_translation);
    
        let (canditate_frags, reference_frags) = match from {
            Language::Japanese | Language::Korean | Language::Chinese => (
                sentence.text.chars().into_iter().map(|c| c.to_string()).collect(),
                reverse_translation.chars().into_iter().map(|c| c.to_string()).collect()
            ),
            _ => (fragments(&sentence.text), fragments(&reverse_translation))
        };
    
        let bleu = bleu_metric(
            &canditate_frags,
            &reference_frags,
            4
        );
        records.push(
            Record{
                from: from.clone(), to: to.clone(),
                original: sentence.text.clone(),
                translation,
                reverse_translation,
                bleu
            }
        );
    }
    records
}
