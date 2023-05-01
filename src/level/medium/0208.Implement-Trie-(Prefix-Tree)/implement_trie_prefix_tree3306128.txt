// https://leetcode.com/problems/implement-trie-prefix-tree/solutions/3306128/rust/
use std::collections::HashMap;

struct Trie {
    is_end: bool,
    children: HashMap<char, Trie>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {

    fn new() -> Self {
        Self {
            is_end: false,
            children: HashMap::new(),
        }        
    }
    
    fn insert(&mut self, word: String) {
        let mut cur = self;

        for c in word.chars() {
            cur = cur.children.entry(c).or_insert(Trie::new());
        }

        cur.is_end = true;
    }
    
    fn search(&self, word: String) -> bool {
        let mut cur = self;
        for c in word.chars() {
            if let Some(child) = cur.children.get(&c) {
                cur = child;
            } else {
                return false;
            }
        }
        cur.is_end
    }
    
    fn starts_with(&self, prefix: String) -> bool {
        let mut cur = self;
        for c in prefix.chars() {
            if let Some(child) = cur.children.get(&c) {
                cur = child;
            } else {
                return false;
            }
        }
        true
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */