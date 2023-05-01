// https://leetcode.com/problems/implement-trie-prefix-tree/solutions/3306445/rust/
use std::collections::HashMap;

struct Trie {
    is_key: bool,
    children: HashMap<char, Trie>,
}

impl Trie {

    fn new() -> Self {
        Self {
            is_key: false,
            children: HashMap::new(),
        }
    }
    
    fn get_child(&self, c: char) -> Option<&Self> {
        self.children.get(&c)
    }

    fn get_or_create_child(&mut self, c: char) -> &mut Self {
        self.children.entry(c).or_insert(Self::new())
    }

    fn insert(&mut self, word: String) {
        let mut root = self;
        for c in word.chars() {
            root = root.get_or_create_child(c)
        }
        root.is_key = true;
    }
    
    fn search(&self, word: String) -> bool {
        let mut root = self;
        for c in word.chars() {
            if let Some(node) = root.get_child(c) {
                root = node;
            } else {
                return false
            }
        }
        root.is_key
    }
    
    fn starts_with(&self, prefix: String) -> bool {
        let mut root = self;
        for c in prefix.chars() {
            if let Some(node) = root.get_child(c) {
                root = node;
            } else {
                return false
            }
        }
        true
    }
}