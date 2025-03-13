use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Trie {
    root: Node,
}

#[derive(Debug, Default)]
pub struct Node {
    children: HashMap<char, Node>,
    is_end: bool
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: Node::default()
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut current = &mut self.root;

        for c in word.chars() {
            current = current.children.entry(c).or_default();
        }

        current.is_end = true;
    }

    pub fn insert_many(&mut self, words: Vec<&str>) {
        for word in words {
            self.insert(word);
        }
    }

    fn traverse(&self, word: &str) -> Option<&Node> {
        let mut current = &self.root;

        for c in word.chars() {
            if let Some(node) = current.children.get(&c) {
                current = node;
            } else {
                return None;
            }
        }

        Some(current)   
    }

    pub fn contains(&self, word: &str) -> bool {
        let node = self.traverse(word);
        node.is_some() && node.unwrap().is_end
    }

    pub fn contains_partial(&self, word: &str) -> bool {
        let node = self.traverse(word);
        node.is_some()
    }
}