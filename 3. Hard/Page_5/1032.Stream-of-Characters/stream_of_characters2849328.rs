// https://leetcode.com/problems/stream-of-characters/solutions/2849328/rust-solution-faster-than-100-time-58-ms-memory-22-8-mb/
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Default)]
struct Node {
    is_leaf : bool,
    children : [Option<Rc<RefCell<Node>>>; 26],
}

impl Node {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            is_leaf : false,
            children : Default::default()
        }
    }
}

#[derive(Debug)]
struct Trie {
    root : Rc<RefCell<Node>>
}

impl Trie {
    #[inline(always)]
    pub fn new() -> Self{
        Self {
            root : Rc::new(RefCell::new(Node::new()))
        }
    }
    
    #[inline(always)]
    pub fn add_words(&self, words : Vec<String>) {
        for word in words {
            self.add_word(word);
        }
    }
    
    pub fn add_word(&self, word : String) {
        let mut curr = Rc::clone(&self.root);
        
        for b in word.into_bytes().into_iter().rev() {
            if curr.borrow_mut().children[(b - b'a') as usize].is_none() {
                curr.borrow_mut().children[(b - b'a') as usize] = Some(Rc::new(RefCell::new(Node::new())));
            }
            let next = Rc::clone(curr.borrow().children[(b - b'a') as usize].as_ref().unwrap());
            curr = next;
        }
        
        curr.borrow_mut().is_leaf = true;
    }
    
    pub fn has_suffix(&self, word : &String) -> bool {
        let mut curr = Rc::clone(&self.root);
        
        for b in word.bytes().rev() {
            let next = if let Some(next) = &curr.borrow().children[(b - b'a') as usize] {
                if next.borrow().is_leaf {
                    return true;
                }
                Rc::clone(next)
            } else {
                return false;
            };
            
            curr = next;
        }
        false
    }
    
}

struct StreamChecker {
    storage : Trie,
    buf : String
}

impl StreamChecker {

    #[inline(always)]
    fn new(words: Vec<String>) -> Self {
        let mut storage = Trie::new();
        storage.add_words(words);
        Self {
            storage,
            buf : String::with_capacity(40_000),
        }
    }
    #[inline(always)]
    fn query(&mut self, letter: char) -> bool {
        self.buf.push(letter);
        let res = self.storage.has_suffix(&self.buf);
        res
    }
}