// https://leetcode.com/problems/binary-search-tree-iterator/solutions/966462/trees-in-rust-are-fun/
use std::rc::Rc;
use std::cell::RefCell;

struct BSTIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>,
}

impl BSTIterator {
    fn go_left(stack: &mut Vec<Rc<RefCell<TreeNode>>>, mut node: Option<Rc<RefCell<TreeNode>>>) {
        while let Some(current) = node {
            stack.push(Rc::clone(&current));
            current.borrow_mut();
            node = current.borrow().left.as_ref().map(|l| Rc::clone(l));
        }
    }

    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut stack = vec![];
        Self::go_left(&mut stack, root);
        Self {
            stack   
        }
    }
    
    fn next(&mut self) -> i32 {
        let nxt_ref = self.stack.pop().unwrap();
        let nxt = nxt_ref.borrow();
        if let Some(right) = nxt.right.as_ref() {
            Self::go_left(&mut self.stack, Some(Rc::clone(right)));
        }
        nxt.val
    }
    
    fn has_next(&self) -> bool {
        self.stack.len() > 0
    }
}