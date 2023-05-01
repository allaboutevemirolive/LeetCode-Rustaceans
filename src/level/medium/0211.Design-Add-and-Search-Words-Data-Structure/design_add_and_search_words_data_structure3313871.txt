// https://leetcode.com/problems/design-add-and-search-words-data-structure/solutions/3313871/rust-elixir-trie-solution/
type OptNode = Option<Box<TrieNode>>;

#[derive(Default)]
struct TrieNode {
    flag: bool,
    next: [OptNode; 26],
}

struct WordDictionary {
    root: OptNode,
}

impl WordDictionary {
    fn new() -> Self {
        Self {
            root: Some(Box::new(Default::default())),
        }
    }
    
    fn add_word(&mut self, word: String) {
        let mut node = &mut self.root;
        for b in word.bytes() {
            let i = (b - b'a') as usize;
            node = &mut node.as_mut().unwrap().next[i];
            if node.is_none() {
                *node = Some(Box::new(Default::default()));
            }
        }
        node.as_mut().unwrap().flag = true;
    }
    
    fn search(&self, word: String) -> bool {
        let mut v = vec![&self.root];
        let mut v2 = Vec::new();
        for b in word.bytes() {
            if b == b'.' {
                for opt in v.drain(..) {
                    for o in opt.as_ref().unwrap().next
                    .iter().filter(|o| o.is_some()) {
                        v2.push(o);
                    }
                }
            }
            else {
                let i = (b - b'a') as usize;
                for opt in v.drain(..) {
                    if opt.as_ref().unwrap().next[i].is_some() {
                        v2.push(&opt.as_ref().unwrap().next[i]);
                    }
                }
            }
            std::mem::swap(&mut v, &mut v2);
        }
        v.iter().any(|o| o.as_ref().unwrap().flag)
    }
}