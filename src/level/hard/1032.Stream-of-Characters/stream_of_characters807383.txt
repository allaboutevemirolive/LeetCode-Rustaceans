// https://leetcode.com/problems/stream-of-characters/solutions/807383/rust-92ms-100-38m/
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Trie {
    is_word: bool,
    next: Vec<Option<Trie>>,
}

impl Trie {
    fn new() -> Self {
        Trie {
            is_word: false,
            next: vec![None; 26],
        }
    }
}

struct StreamChecker {
    root: Trie,
    s: String,
    max: usize,
}

impl StreamChecker {
    fn new(words: Vec<String>) -> Self {
        let mut root = Trie::new();
        let mut max = 0;
        for w in words {
            if w.len() > max {
                max = w.len()
            }
            let v = w.chars().rev().collect::<String>();
            let mut node = &mut root;
            for ch in v.chars() {
                if node.next[(ch as u8 - b'a') as usize].is_none() {
                    node.next[(ch as u8 - b'a') as usize] = Some(Trie::new())
                }
                node = node.next[(ch as u8 - b'a') as usize].as_mut().unwrap();
            }
            node.is_word = true;
        }
        let s = String::new();
        StreamChecker { root, s, max }
    }

    fn query(&mut self, letter: char) -> bool {
        self.s.push(letter);
        let mut node = &mut self.root;
        let mut start = 0;
        if self.s.len() > self.max {
            start = self.s.len() - self.max;
        };
        for i in (start..self.s.len()).rev() {
            let ch = self.s.as_bytes()[i] as char;
            if let Some(n) = &mut node.next[(ch as u8 - b'a') as usize] {
                if n.is_word {
                    return true;
                }
                node = n
            } else {
                return false;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stream_checker() {
        let mut sc = StreamChecker::new(vec!["cd".to_string(), "f".to_string(), "kl".to_string()]);
        assert!(!sc.query('a')); // return false
        assert!(!sc.query('b')); // return false
        assert!(!sc.query('c')); // return false
        assert!(sc.query('d')); // return true, because 'cd' is in the wordlist
        assert!(!sc.query('e')); // return false
        assert!(sc.query('f')); // return true, because 'f' is in the wordlist
        assert!(!sc.query('g')); // return false
        assert!(!sc.query('h')); // return false
        assert!(!sc.query('i')); // return false
        assert!(!sc.query('j')); // return false
        assert!(!sc.query('k')); // return false
        assert!(sc.query('l')); // return true, because 'kl' is in the wordlist
    }
}