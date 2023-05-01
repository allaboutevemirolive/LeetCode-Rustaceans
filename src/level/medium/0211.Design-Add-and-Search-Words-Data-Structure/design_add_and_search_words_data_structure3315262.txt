// https://leetcode.com/problems/design-add-and-search-words-data-structure/solutions/3315262/rust-solution/
struct WordDictionary {
    trie_node: TrieNode,
}

impl WordDictionary {
    fn new() -> Self {
        WordDictionary {
            trie_node: TrieNode::new(),
        }
    }

    fn add_word(&mut self, word: String) {
        self.trie_node.add(word.as_bytes());
    }

    fn search(&self, word: String) -> bool {
        self.trie_node.search(word.as_bytes())
    }
}

struct TrieNode {
    is_term: bool,
    children: [Option<Box<TrieNode>>; 26],
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            is_term: false,
            children: Default::default(),
        }
    }

    fn add(&mut self, word: &[u8]) {
        let mut current = self;
        for &b in word {
            let i = usize::from(b - b'a');
            current = current.children[i].get_or_insert(Box::new(TrieNode::new()));
        }
        current.is_term = true;
    }

    fn search(&self, word: &[u8]) -> bool {
        if word.is_empty() {
            self.is_term
        } else {
            if word[0] == b'.' {
                for b in b'a'..=b'z' {
                    if let Some(next) = &self.children[usize::from(b - b'a')] {
                        if next.search(&word[1..]) {
                            return true;
                        }
                    }
                }
                false
            } else {
                if let Some(next) = &self.children[usize::from(word[0] - b'a')] {
                    next.search(&word[1..])
                } else {
                    false
                }
            }
        }
    }
}