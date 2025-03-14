#![allow(unused)]

use dict::{Dictionary, Frequency};
use model::trie;

mod common;

mod dict;
mod model;
mod find;

pub const SIZE: usize = 3;
pub const TOP_N: u64 = 0;

fn main() {
    let freq =
        dict::freq_from_csv("res/frequency.csv".into())
        .expect("Could not load frequency.")
        .into_iter()
        .collect::<Frequency>();

    let dict: Vec<String> = 
        dict::dict_from_csv("res/dictionary.csv".into())
        .expect("Could not load dictionary.")
        .into_iter()
        .filter(|word| word.len() == SIZE)
        .collect::<Dictionary>();
    
    let mut trie = trie::Trie::new();
    trie.insert_many(dict.clone());

    let mut solver = find::Solver::new(trie);
    solver.solve();
}
