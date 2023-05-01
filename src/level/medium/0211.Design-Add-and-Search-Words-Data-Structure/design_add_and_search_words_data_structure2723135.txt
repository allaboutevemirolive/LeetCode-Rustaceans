// https://leetcode.com/problems/design-add-and-search-words-data-structure/solutions/2723135/rust-trie/
const N_LETTERS: usize = (b'z' - b'a' + 1) as _;

#[derive(Default)]
struct TrieNode {
    word: bool,
    children: [usize; N_LETTERS],
}

struct WordDictionary {
    nodes: Vec<TrieNode>
}

impl WordDictionary {

    fn new() -> Self {
        Self { nodes: vec![TrieNode::default()] }
    }
    
    fn add_word(&mut self, word: String) {
        let mut i = 0;
        for b in word.bytes() {
            let c = (b - b'a') as usize;
            if self.nodes[i].children[c] == 0 {
                self.nodes[i].children[c] = self.nodes.len();
                self.nodes.push(TrieNode::default());
            }
            i = self.nodes[i].children[c];
        }
        self.nodes[i].word = true;
    }
    
    fn search(&self, word: String) -> bool {
        let word = word.as_bytes();
        let mut stack = vec![(0, 0)];

        while let Some((i, j)) = stack.pop() {
            let node = &self.nodes[i];
            if j == word.len() {
                if node.word {
                    return true;
                }
            } else {
                let b = word[j];
                if b == b'.' {
                    stack.extend(node.children.iter().filter_map(|c| (*c != 0).then(|| (*c, j+1))));
                } else {
                    let c = node.children[(b - b'a') as usize];
                    if c != 0 {
                        stack.push((c, j+1));
                    }
                }
            }
        }
        false
    }
}