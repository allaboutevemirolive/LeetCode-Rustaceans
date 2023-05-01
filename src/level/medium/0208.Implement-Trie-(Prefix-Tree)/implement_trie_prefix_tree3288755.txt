// https://leetcode.com/problems/implement-trie-prefix-tree/solutions/3288755/rust-solution/
#[derive(Debug, Clone)]
struct TrieNode {
    links: Vec<Option<TrieNode>>,
    is_end: bool,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            links: vec![None; 26],
            is_end: false,
        }
    }

    fn contains_key(&self, ch: char) -> bool {
        self.links[ch as usize - 'a' as usize].is_some()
    }

    fn get(&self, ch: char) -> Option<&TrieNode> {
        self.links[ch as usize - 'a' as usize].as_ref()
    }

    fn get_mut(&mut self, ch: char) -> Option<&mut TrieNode> {
        self.links[ch as usize - 'a' as usize].as_mut()
    }

    fn put(&mut self, ch: char, node: TrieNode) {
        self.links[ch as usize - 'a' as usize] = Some(node);
    }
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            if !node.contains_key(ch) {
                node.put(ch, TrieNode::new());
            }
            node = node.get_mut(ch).unwrap();
        }
        node.is_end = true;
    }

    fn search(&self, word: String) -> bool {
        let mut node = &self.root;
        for ch in word.chars() {
            if node.contains_key(ch) {
                node = node.get(ch).unwrap();
            } else {
                return false;
            }
        }
        node.is_end
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut node = &self.root;
        for ch in prefix.chars() {
            if node.contains_key(ch) {
                node = node.get(ch).unwrap();
            } else {
                return false;
            }
        }
        true
    }
}