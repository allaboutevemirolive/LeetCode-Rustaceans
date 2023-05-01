// https://leetcode.com/problems/design-add-and-search-words-data-structure/solutions/3321490/rust-trie/
use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    is_word: bool, 
    children: HashMap<char, Node>
}

impl Node {
    fn new() -> Self {
        Self {
            is_word: false,    
            children: HashMap::new(),
        }
    }
}

struct WordDictionary {
    root: Node
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {

    fn new() -> Self {
        Self {
            root: Node::new(),
        }
    }
    
    fn add_word(&mut self, word: String) {
        let node = word
            .chars()
            .fold(&mut self.root, |node, c| {
                node.children.entry(c).or_insert(Node::new())
            }); 
        node.is_word = true;
    }
    
    fn dfs(node: &Node, vs: &[char]) -> bool {
        match vs {
            [] => node.is_word,
            ['.', rest @ ..] => node
                .children
                .values()
                .any(|v| Self::dfs(v, rest)),
            [c, rest @ ..] => {
                match node.children.get(c) {
                    Some(nn) => Self::dfs(nn, rest),
                    None => false
                }
            }
        }
    }

    fn search(&self, word: String) -> bool {
        let v = word 
            .chars() 
            .collect::<Vec<_>>();
        Self::dfs(&self.root, &v)
    }
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */