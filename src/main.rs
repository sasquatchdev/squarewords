#![allow(unused)]

use std::{thread::sleep, time::Duration};

use dict::{Dictionary, Frequency};
use model::trie;

mod common;

mod dict;
mod model;
mod find;

pub const SIZE: usize = 4;
pub const TOP_N: u64 = 0;
pub const UNIQUE: bool = true;
pub const LIMIT: usize = 10;

fn main() {
    let start = std::time::Instant::now();

    let freq =
        dict::freq_from_csv("res/frequency.csv".into())
        .expect("Could not load frequency.")
        .into_iter()
        .filter(|(_, place)| place <= &TOP_N || TOP_N == 0)
        .collect::<Frequency>();

    let dict: Vec<String> = 
        dict::dict_from_csv("res/dictionary.csv".into())
        .expect("Could not load dictionary.")
        .into_iter()
        .filter(|word| word.len() == SIZE)
        .filter(|word| freq.iter().any(|(f_word, _)| f_word == word) || TOP_N == 0)
        .collect::<Dictionary>();
    
    let mut trie = trie::Trie::new();
    trie.insert_many(dict.clone());

    let mut solver = find::Solver::new(trie);
    solver.solve();

    let end = std::time::Instant::now();
    println!("Elapsed time: {:?} secs.", (end - start).as_secs());

    sleep(Duration::from_secs(1));

    for solution in solver.solutions() {
        print(&solution);
    }

    if solver.solutions().len() == 0 {
        println!("No solution found.");
    }
}

fn print(solution: &[char; SIZE * SIZE]) {
    for i in 0..SIZE {
        for j in 0..SIZE {
            print!("{} ", solution[i * SIZE + j]);
        }
        println!();
    }
    println!();
}
