use std::{path::PathBuf, task::ready};
use crate::common::error::Result;

pub type Dictionary = Vec<String>;
pub type Frequency = Vec<(String, u64)>;

pub fn dict_from_csv(path: PathBuf) -> Result<Dictionary> {
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

pub fn freq_from_csv(path: PathBuf) -> Result<Frequency> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut freq = Vec::new();

    let mut n = 1;
    for result in reader.records() {
        let record = result?;
        if let Some(word) = record.get(0) {
            freq.push((word.to_string(), n));
            n += 1;
        }
    }

    Ok(freq)
}