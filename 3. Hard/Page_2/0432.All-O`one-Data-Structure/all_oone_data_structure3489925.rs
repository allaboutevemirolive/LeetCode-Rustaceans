// https://leetcode.com/problems/all-oone-data-structure/solutions/3489925/rust-o-1/
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

struct Node {
    count: i32,
    strings: HashSet<String>,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>
}

impl Node {
    fn new(count: i32) -> Self {
        Self {
            count,
            strings: HashSet::new(),
            left: None,
            right: None
        }
    }
}


struct AllOne {
    dummy_head: Rc<RefCell<Node>>,
    dummy_tail: Rc<RefCell<Node>>,
    string_to_node: HashMap<String, Rc<RefCell<Node>>>
}


impl AllOne {

    fn new() -> Self {
        let dummy_head = Rc::new(RefCell::new(Node::new(0)));
        let dummy_tail = Rc::new(RefCell::new(Node::new(-1)));
        dummy_head.borrow_mut().right = Some(dummy_tail.clone());
        dummy_tail.borrow_mut().left = Some(dummy_head.clone());

        Self {
            dummy_head,
            dummy_tail,
            string_to_node: HashMap::new()
        }
    }

    fn add_after(&mut self, node: Rc<RefCell<Node>>, key: String) -> Rc<RefCell<Node>> {
        let left = node;
        let right = left.borrow().right.as_ref().unwrap().clone();

        if left.borrow().count == right.borrow().count - 1 {
            right.borrow_mut().strings.insert(key);
            right
        }
        else {
            let new_node = Rc::new(RefCell::new(Node::new(left.borrow().count + 1)));
            new_node.borrow_mut().strings.insert(key);
            new_node.borrow_mut().left = Some(left.clone());
            new_node.borrow_mut().right = Some(right.clone());
            left.borrow_mut().right = Some(new_node.clone());
            right.borrow_mut().left = Some(new_node.clone());
            new_node
        }
    }

    fn add_before(&mut self, node: Rc<RefCell<Node>>, key: String) -> Rc<RefCell<Node>> {
        let right = node;
        let left = right.borrow().left.as_ref().unwrap().clone();

        if left.borrow().count == right.borrow().count - 1 {
            left.borrow_mut().strings.insert(key);
            left
        }
        else {
            let new_node = Rc::new(RefCell::new(Node::new(right.borrow().count - 1)));
            new_node.borrow_mut().strings.insert(key);
            new_node.borrow_mut().left = Some(left.clone());
            new_node.borrow_mut().right = Some(right.clone());
            left.borrow_mut().right = Some(new_node.clone());
            right.borrow_mut().left = Some(new_node.clone());
            new_node
        }
    }

    fn inc(&mut self, key: String) {
        let current_node;
        if let Some(node) = self.string_to_node.get(&key) {
            current_node = node.clone();
            current_node.borrow_mut().strings.remove(&key);
        }
        else {
            current_node = self.dummy_head.clone();
        }

        let new_node = self.add_after(current_node.clone(), key.clone());
        self.string_to_node.insert(key, new_node);

        if current_node.borrow().count > 0 {
            self.remove_if_empty(current_node);
        }
    }

    fn remove_if_empty(&mut self, node: Rc<RefCell<Node>>) {
        if node.borrow().strings.len() != 0 {
            return;
        }

        let left = node.borrow().left.as_ref().unwrap().clone();
        let right = node.borrow().right.as_ref().unwrap().clone();

        left.borrow_mut().right = Some(right.clone());
        right.borrow_mut().left = Some(left.clone());
    }

    fn dec(&mut self, key: String) {
        let node = self.string_to_node.get(&key).unwrap().clone();
        if node.borrow_mut().count == 1 {
            node.borrow_mut().strings.remove(&key);
            self.string_to_node.remove(&key);
        }
        else {
            node.borrow_mut().strings.remove(&key);
            let new_node = self.add_before(node.clone(), key.clone());
            self.string_to_node.insert(key, new_node);
        }

        self.remove_if_empty(node);
    }

    fn get_from_node(&self, node: Rc<RefCell<Node>>) -> String {
        if node.borrow().count <= 0 {
            "".to_string()
        }
        else {
            node.borrow().strings.iter().nth(0).unwrap().to_string() 
        }
    }

    fn get_max_key(&self) -> String {
        self.get_from_node(self.dummy_tail.borrow().left.as_ref().unwrap().clone())
    }

    fn get_min_key(&self) -> String {
        self.get_from_node(self.dummy_head.borrow().right.as_ref().unwrap().clone())
    }
}
