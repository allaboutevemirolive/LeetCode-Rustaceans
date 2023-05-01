// https://leetcode.com/problems/binary-search-tree-iterator/solutions/1511355/rust-stack-8-ms/
use std::rc::Rc;
use std::cell::RefCell;

struct BSTIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>
}

impl BSTIterator {

    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        let mut left: Option<Rc<RefCell<TreeNode>>> = root;
        while !left.is_none() {
            stack.push(left.as_ref().unwrap().clone());
            left = left.unwrap().borrow().left.clone();
        }
        BSTIterator { stack: stack }
    }
    
    fn next(&mut self) -> i32 {
        match self.stack.pop() {
            Some(leaf) => {
                let next_val = leaf.borrow().val;
                
                let mut child: Option<Rc<RefCell<TreeNode>>> = leaf.borrow().right.clone();
                while !child.is_none() {
                    self.stack.push(child.as_ref().unwrap().clone());
                    child = child.unwrap().borrow().left.clone();
                }
                
                next_val
            },
            None => i32::MIN
        }
    }
    
    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}