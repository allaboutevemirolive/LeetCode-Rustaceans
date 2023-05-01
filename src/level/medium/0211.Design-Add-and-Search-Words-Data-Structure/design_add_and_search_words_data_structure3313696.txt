// https://leetcode.com/problems/design-add-and-search-words-data-structure/solutions/3313696/rust-solution/
struct TrieNode {
    matches: [Option<Box<TrieNode>>; 26],
    reach: usize,
    branches: usize,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            matches: [
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None,
            ],
            reach: 0,
            branches: 0,
        }
    }

    fn add_word(&mut self, word: &[u8]) {
        if word.len() == 0 {
            return self.reach += 1;
        }
        self.reach += 1;
        self.branches += 1;
        let pos = (word[0] - b'a') as usize;

        if let None = self.matches[pos] {
            self.matches[pos] = Some(Box::new(TrieNode::new()));
        }

        self.matches[pos]
            .as_deref_mut()
            .unwrap()
            .add_word(&word[1..])
    }

    fn search(&self, word: &[u8]) -> bool {
        if word.len() == 0 {
            return self.reach > self.branches;
        } else {
            if word[0] == b'.' {
                for v in &self.matches {
                    if let Some(ref v) = v {
                        if v.search(&word[1..]) {
                            return true;
                        }
                    }
                }
                false
            } else {
                let pos = (word[0] - b'a') as usize;
                match self.matches[pos] {
                    None => false,
                    Some(ref v) => v.search(&word[1..]),
                }
            }
        }
    }
}

struct WordDictionary(TrieNode);

impl WordDictionary {
    fn new() -> Self {
        Self(TrieNode::new())
    }

    fn add_word(&mut self, word: String) {
        self.0.add_word(word.as_bytes())
    }

    fn search(&self, word: String) -> bool {
        self.0.search(word.as_bytes())
    }
}
