// https://leetcode.com/problems/design-add-and-search-words-data-structure/solutions/3307803/rust-recursive-trie/
#[derive(Clone, Debug)]
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
    fn contains(&self, key: char) -> bool {
        self.links[key as usize - 'a' as usize].is_some()
    }
    fn get_mut(&mut self, key: char) -> Option<&mut TrieNode> {
        self.links[key as usize - 'a' as usize].as_mut()
    }
    fn get(&self, key: char) -> Option<&TrieNode> {
        self.links[key as usize - 'a' as usize].as_ref()
    }
    fn put(&mut self, key: char, value: TrieNode) {
        self.links[key as usize - 'a' as usize] = Some(value);
    }
}

struct WordDictionary {
    root: TrieNode,
}
impl WordDictionary {
    fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }
    fn add_word(&mut self, word: String) {
        let mut root = &mut self.root;
        for ch in word.chars() {
            if !root.contains(ch) {
                root.put(ch, TrieNode::new());
            }
            root = root.get_mut(ch).unwrap();
        }
        root.is_end = true;
    }
    fn search(&self, word: String) -> bool {
        self.search_node(word.as_str(), &self.root)
    }
    fn search_node(&self, word: &str, from: &TrieNode) -> bool {
        if word.len() == 1 {
            let ch = word.chars().next().unwrap();
            return if ch == '.' {
                from.links.iter().flatten().filter(|x| x.is_end).count() > 0
            } else {
                from.contains(ch) && from.get(ch).unwrap().is_end
            };
        }
        let mut found = false;
        let ch = word.chars().next().unwrap();
        if ch == '.' {
            for n in from.links.iter().flatten() {
                found |= self.search_node(&word[1..], n)
            }
        } else {
            found |= from.contains(ch) && self.search_node(&word[1..], from.get(ch).unwrap());
        }
        found
    }
}