use std::collections::HashSet;

use crate::{model::trie::Trie, SIZE, UNIQUE};

pub struct Solver {
    trie: Trie,
    chars: [char; SIZE * SIZE],
}

impl Solver {
    pub fn new(trie: Trie) -> Self {
        Self { trie, chars: [' '; SIZE * SIZE] }
    }

    pub fn solve(&mut self, callback: &mut impl FnMut(&[char; SIZE * SIZE])) {
        self.solve_rec(0, callback);
    }

    pub fn solve_rec(&mut self, index: usize, callback: &mut impl FnMut(&[char; SIZE * SIZE])) {
        if index == SIZE * SIZE {
            if self.valid() {
                if !UNIQUE || self.unique() {
                    callback(&self.chars);
                }
            }
            return;
        }

        for c in 'a' ..= 'z' {
            self.chars[index] = c;
            if self.valid() {
                self.solve_rec(index + 1, callback);
            }
        }

        self.chars[index] = ' ';
    }

    fn unique(&self) -> bool {
        let mut set = HashSet::new();
        for word in self.words() {
            if !set.insert(word) {
                return false;
            }
        }

        true
    }

    fn valid(&self) -> bool {
        for i in 0..SIZE {
            if !self.valid_row(i) || !self.valid_col(i) {
                return false;
            }
        }
        true
    }

    fn rows(&self) -> Vec<String> {
        (0..SIZE).map(|row| self.row(row)).collect()
    }

    fn cols(&self) -> Vec<String> {
        (0..SIZE).map(|col| self.col(col)).collect()
    }

    fn words(&self) -> Vec<String> {
        let mut words = self.rows();
        words.extend(self.cols());
        words
    }

    fn row(&self, row: usize) -> String {
        let mut chars = [' '; SIZE];
        for i in 0..SIZE {
            chars[i] = self.chars[row * SIZE + i];
        }
        let word = chars.iter().collect::<String>();
        let word = word.trim().to_string();
        word
    }

    fn col(&self, col: usize) -> String {
        let mut chars = [' '; SIZE];
        for i in 0..SIZE {
            chars[i] = self.chars[i * SIZE + col];
        }
        let word = chars.iter().collect::<String>();
        let word = word.trim().to_string();
        word
    }

    fn valid_row(&self, row: usize) -> bool {
        let word = self.row(row);
        let valid = self.trie.contains_partial(word);

        valid
    }

    fn valid_col(&self, col: usize) -> bool {
        let word = self.col(col);
        let valid = self.trie.contains_partial(word);

        valid
    }
}