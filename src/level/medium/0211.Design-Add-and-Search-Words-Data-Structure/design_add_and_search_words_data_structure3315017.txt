// https://leetcode.com/problems/design-add-and-search-words-data-structure/solutions/3315017/rust-iterative-trie/
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
    fn contains(&self, key: char) -> bool {
        self.links[key as usize - 'a' as usize].is_some()
    }
    fn put(&mut self, key: char, value: TrieNode) {
        self.links[key as usize - 'a' as usize] = Some(value);
    }
    fn get(&self, key: char) -> Option<&TrieNode> {
        self.links[key as usize - 'a' as usize].as_ref()
    }
    fn get_mut(&mut self, key: char) -> Option<&mut TrieNode> {
        self.links[key as usize - 'a' as usize].as_mut()
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
}
struct WordDictionary {
    trie: Trie,
}
impl WordDictionary {
    fn new() -> Self {
        Self { trie: Trie::new() }
    }
    fn add_word(&mut self, word: String) {
        self.trie.add_word(word);
    }
    fn search(&self, word: String) -> bool {
        let word = word.chars().collect::<Vec<_>>();
        let mut stack = vec![];

        stack.push((&self.trie.root, 0usize));

        let mut found = false;
        while !stack.is_empty() {
            let (node, word_idx) = stack.pop().unwrap();
            let cur_char = word[word_idx];

            if word_idx == word.len() - 1 {
                if cur_char == '.' {
                    found |= node.links.iter().flatten().filter(|x| x.is_end).count() > 0;
                } else if node.contains(cur_char) {
                    found |= node.get(cur_char).unwrap().is_end;
                }
                continue;
            }

            if cur_char == '.' {
                node.links
                    .iter()
                    .flatten()
                    .for_each(|x| stack.push((x, word_idx + 1)));
            } else if node.contains(cur_char) {
                stack.push((node.get(cur_char).unwrap(), word_idx + 1));
            }
        }

        found
    }
}