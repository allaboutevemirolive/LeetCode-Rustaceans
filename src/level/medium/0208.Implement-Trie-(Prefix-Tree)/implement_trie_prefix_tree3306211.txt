// https://leetcode.com/problems/implement-trie-prefix-tree/solutions/3306211/rust-solution/
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct TrieNode {
    links: HashMap<char, Box<TrieNode>>,
    is_end: bool,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            links: HashMap::new(),
            is_end: false,
        }
    }

    fn contains(&self, c: char) -> bool {
        self.links.get(&c).is_some()
    }

    fn set(&mut self, c: char, node: Box<TrieNode>) {
        self.links.insert(c, node);
    }

    fn get_mut(&mut self, c: char) -> Option<&mut Box<TrieNode>> {
        self.links.get_mut(&c)
    }

    fn get(&self, c: char) -> Option<&Box<TrieNode>> {
        self.links.get(&c)
    }

    fn is_end(&self) -> bool {
        self.is_end
    }

    fn set_end(&mut self) {
        self.is_end = true
    }
}

struct Trie {
    root: Box<TrieNode>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Self {
            root: Box::new(TrieNode::new()),
        }
    }

    fn insert(&mut self, word: String) {
        let mut node = self.root.as_mut();
        for chr in word.chars().into_iter() {
            node = {
                if !node.contains(chr) {
                    node.set(chr, Box::new(TrieNode::new()));
                }
                node.get_mut(chr).unwrap()
            }
        }

        node.set_end();
    }

    fn search(&self, word: String) -> bool {
        let mut node = self.root.as_ref();
        for chr in word.chars().into_iter() {
            node = {
                if !node.contains(chr) {
                    return false;
                }

                node.get(chr).unwrap()
            }
        }

        node.is_end()
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut node = self.root.as_ref();
        for chr in prefix.chars().into_iter() {
            node = {
                if !node.contains(chr) {
                    return false;
                }

                node.get(chr).unwrap()
            }
        }

        true
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */

#[cfg(test)]
mod tests {
    use super::Trie;

    #[test]
    fn test() {
        let mut tree = Trie::new();
        tree.insert("apple".into());
        assert_eq!(tree.search("apple".into()), true);
        assert_eq!(tree.search("app".into()), false);
        assert_eq!(tree.starts_with("app".into()), true);
        tree.insert("app".into());
        assert_eq!(tree.search("app".into()), true);
    }
}
