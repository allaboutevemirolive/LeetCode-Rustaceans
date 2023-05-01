// https://leetcode.com/problems/design-add-and-search-words-data-structure/solutions/3313825/rust/
#[derive(Clone)]
pub struct Trie {
    trie_c: Vec<Option<Box<Trie>>>,
    is_end: bool
}

pub struct WordDictionary {
    root: Option<Box<Trie>>
}

impl Trie {
    fn new() -> Self {
        Trie {
            trie_c: vec![None; 26],
            is_end: false
        }
    }
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {

    fn new() -> Self {
        WordDictionary {
            root: Some(Box::new(Trie::new()))
        }
    }
    
    fn add_word(&mut self, word: String) {
        let mut curr = self.root.as_mut();
        for c in word.chars() {
            let index = c as usize - 'a' as usize;
            if let Some(trie_node) = curr {
                if trie_node.trie_c[index].is_none() {
                    trie_node.trie_c[index] = Some(Box::new(Trie::new()));
                }
                curr = trie_node.trie_c[index].as_mut();
            }
        }
        if let Some(trie_node) = curr {
            trie_node.is_end = true;
        }
    }
    
    fn search(&self, word: String) -> bool {
        let str_c: Vec<char> = word.chars().collect();
        dfs(self.root.as_ref(), 0, &str_c)
    }
}

pub fn dfs(curr: Option<&Box<Trie>>, index: usize, str_c: &Vec<char>) -> bool {
    match curr {
        Some(trie_node) => {
             if index == str_c.len() {
                 trie_node.is_end
             } else if str_c[index] == '.' {
                 (0..26_usize).any(|z| {
                    dfs(trie_node.trie_c[z].as_ref(), index + 1, str_c)
                })
             } else {
                dfs(trie_node.trie_c[str_c[index] as usize - 'a' as usize].as_ref(), index + 1, str_c)
             }
        },
        None => false
    }
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */