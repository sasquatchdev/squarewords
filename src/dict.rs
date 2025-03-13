use std::path::PathBuf;
use crate::common::error::Result;

pub type Dictionary = Vec<String>;

pub fn from_csv(path: PathBuf) -> Result<Dictionary> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut dict = Vec::new();

    for result in reader.records() {
        let record = result?;
        if let Some(word) = record.get(0) {
            dict.push(word.to_string());
        }
    }

    Ok(dict)
}