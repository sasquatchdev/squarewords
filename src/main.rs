#![allow(unused)]

use std::{sync::{mpsc, Arc, Mutex}, thread::{self, sleep}, time::Duration};

use dict::Dictionary;
use model::trie;

mod common;

mod dict;
mod model;
mod find;

pub const SIZE: usize = 3;
pub const TOP_N: u64 = 100;
pub const UNIQUE: bool = true;
pub const LIMIT: usize = 0;

fn main() {
    let freq = load_freq();    
    let dict = load_dict(&freq);

    let mut trie = trie::Trie::new();
    trie.insert_many(dict.clone());

    let (tx, rx) = mpsc::channel();
    let trie = Arc::new(Mutex::new(trie));

    let producer = {
        let trie = Arc::clone(&trie);
        let tx = tx.clone();

        thread::spawn(move || {
            let mut solver = find::Solver::new(trie.lock().unwrap().clone());
            solver.solve(&mut |solution| {
                tx.send(solution.to_vec()).expect("Failed to send solution.");
            });
        })
    };

    let consumer = thread::spawn(move || {
        print_all(rx);
    });

    producer.join().expect("Failed to join producer thread.");
    consumer.join().expect("Failed to join consumer thread.");
}

fn load_freq() -> Dictionary {
    dict::freq_from_csv("res/frequency.csv".into())
        .expect("Could not load frequency.")
        .into_iter()
        .filter(|word| word.len() == SIZE)
        .collect::<Dictionary>()[..TOP_N as usize]
        .to_vec()
}

fn load_dict(freq: &Dictionary) -> Dictionary {
    dict::dict_from_csv("res/dictionary.csv".into())
        .expect("Could not load dictionary.")
        .into_iter()
        .filter(|word| word.len() == SIZE)
        .filter(|word| freq.iter().any(|f_word| f_word == word) || TOP_N == 0)
        .collect::<Dictionary>()
}

fn print_all(rx: mpsc::Receiver<Vec<char>>) {
    for solution in rx {
        print(solution);
    }
}

fn print(solution: Vec<char>) {
    for i in 0..SIZE {
        for j in 0..SIZE {
            print!("{} ", solution[i * SIZE + j]);
        }
        println!();
    }
    println!();
}
