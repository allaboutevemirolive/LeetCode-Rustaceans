// https://leetcode.com/problems/binary-search-tree-iterator/solutions/1965256/rust-with-a-stack/
use std::rc::Rc;
use std::cell::RefCell;

struct BSTIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>,
}

impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut iter = Self { stack: Vec::new() };
        iter.fill(root);
        iter
    }

    fn fill(&mut self, node: Option<Rc<RefCell<TreeNode>>>) {
        if let Some(n) = node {
            let left = n.borrow_mut().left.take();
            self.stack.push(n);
            self.fill(left);
        }
    }
	
	fn fill_iterative(&mut self, mut node: Option<Rc<RefCell<TreeNode>>>) {
        while let Some(n) = node {
            let left = n.borrow_mut().left.take();
            self.stack.push(n);
            node = left;
        }
    }
    
    fn next(&mut self) -> i32 {
        let node = self.stack.pop().unwrap();
        let mut borrow = node.borrow_mut();
        self.fill(borrow.right.take());
        borrow.val
    }
    
    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}