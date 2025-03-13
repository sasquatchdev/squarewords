use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Trie {
    root: Node,
}

#[derive(Debug, Default, Clone)]
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
    pub fn from_node(node: Node) -> Self {
        Trie {
            root: node
        }
    }

    pub fn insert(&mut self, word: String) {
        let mut current = &mut self.root;

        for c in word.chars() {
            current = current.children.entry(c).or_default();
        }

        current.is_end = true;
    }

    pub fn insert_many(&mut self, words: Vec<impl Into<String>>) {
        for word in words {
            self.insert(word.into());
        }
    }

    fn traverse(&self, word: String) -> Option<&Node> {
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

    pub fn contains(&self, word: String) -> bool {
        let node = self.traverse(word);
        node.is_some() && node.unwrap().is_end
    }

    pub fn contains_partial(&self, word: String) -> bool {
        let node = self.traverse(word);
        node.is_some()
    }

    pub fn words(&self) -> Vec<String> {
        let mut results = Vec::new();
        self.dfs(&self.root, "".to_string(), &mut results);
        results
    }

    pub fn words_with_prefix(&self, prefix: String) -> Vec<String> {
        let mut results = Vec::new();
        let node = match self.traverse(prefix.clone()) {
            Some(node) => node,
            None => return results
        };

        self.dfs(node, prefix, &mut results);
        results
    }

    fn dfs(&self, node: &Node, prefix: String, results: &mut Vec<String>) {
        if node.is_end {
            results.push(prefix.to_string());
        }

        for (c, child) in &node.children {
            let mut prefix = prefix.clone();
            prefix.push(*c);
            self.dfs(child, prefix, results);
        }
    }
}