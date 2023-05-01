// https://leetcode.com/problems/implement-trie-prefix-tree/solutions/3306005/my-rust-solution/
struct TrieNode {
    items: [Option<Box<TrieNode>>; 26],
    size: usize,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            items: [
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None,
            ],
            size: 0,
        }
    }

    fn len(&self) -> usize {
        let mut ans = 0;
        for item in &self.items {
            if let Some(ref v) = item {
                ans += v.size;
            }
        }
        ans
    }

    fn insert(&mut self, word: &[u8]) {
        self.size += 1;
        if word.len() == 0 {
            return;
        }
        let pos = (word[0] - b'a') as usize;

        if let None = self.items[pos] {
            self.items[pos] = Some(Box::new(TrieNode::new()));
        }

        self.items[pos].as_deref_mut().unwrap().insert(&word[1..])
    }

    fn search(&self, word: &[u8]) -> bool {
        if word.len() == 0 {
            return self.size - self.len() > 0;
        }
        let pos = (word[0] - b'a') as usize;

        if let Some(ref v) = self.items[pos] {
            return v.search(&word[1..]);
        }

        false
    }

    fn starts_with(&self, pref: &[u8]) -> bool {
        if pref.len() == 0 {
            return true;
        }
        let pos = (pref[0] - b'a') as usize;

        if let Some(ref v) = self.items[pos] {
            return v.starts_with(&pref[1..]);
        }

        false
    }
}

struct Trie {
    head: TrieNode,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Self {
            head: TrieNode::new(),
        }
    }

    fn insert(&mut self, word: String) {
        self.head.insert(word.as_bytes())
    }

    fn search(&self, word: String) -> bool {
        self.head.search(word.as_bytes())
    }

    fn starts_with(&self, prefix: String) -> bool {
        self.head.starts_with(prefix.as_bytes())
    }
}