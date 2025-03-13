#![allow(unused)]

use model::trie;

mod common;

mod dict;
mod model;

pub const N: usize = 3;

fn main() {
    let dict = 
        dict::from_csv("dictionary.csv".into())
        .expect("Could not load dictionary.")
        .into_iter()
        .filter(|word| word.len() == N)
        .collect::<Vec<String>>();
    
    let mut trie = trie::Trie::new();
    trie.insert_many(dict);
}
