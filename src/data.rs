use std::{fs::File, io::BufReader, path::Path};

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Language{
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
pub struct Sentence{
    id: u32,
    pub lang: Language,
    pub text: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataSet{
    pub sentences: Vec<Sentence>,
}

pub fn read_user_from_file<P: AsRef<Path>>(path: P) -> Option<DataSet> {
    File::open(path)
        .ok()
        .map(|file| BufReader::new(file))
        .and_then(|r| serde_json::from_reader(r).ok())
}