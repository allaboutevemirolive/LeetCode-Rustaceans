// https://leetcode.com/problems/stream-of-characters/solutions/807292/rust-trie-with-reversed-words/
use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Debug, Default)]
struct Trie {
    children: HashMap<char, Trie>,
    end: bool,
}

impl Trie {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, word: String) {
        let mut link = self;
        for c in word.chars().rev() {
            link = link.children.entry(c).or_default();
        }
        link.end = true;
    }

    fn search(&self, stream: &[char]) -> bool {
        let mut link = self;
        for c in stream.iter().rev() {
            if let Some(next) = link.children.get(&c) {
                link = next;
                if next.end {
                    return true;
                }
            } else {
                return false;
            }
        }
        false
    }
}

struct StreamChecker {
    trie: Trie,
    stream: Vec<char>,
}

impl StreamChecker {
    fn new(words: Vec<String>) -> Self {
        let mut trie = Trie::new();
        for s in words {
            trie.insert(s);
        }
        let stream = vec![];
        StreamChecker { trie, stream }
    }

    fn query(&mut self, letter: char) -> bool {
        self.stream.push(letter);
        self.trie.search(&self.stream)
    }
}