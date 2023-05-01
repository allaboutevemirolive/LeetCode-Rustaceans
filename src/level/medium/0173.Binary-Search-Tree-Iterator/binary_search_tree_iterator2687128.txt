// https://leetcode.com/problems/binary-search-tree-iterator/solutions/2687128/rust-simple-iterative-in-order-traversal/
use std::cell::RefCell;
use std::rc::Rc;

struct BSTIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>,
    cur: Option<Rc<RefCell<TreeNode>>>,
}

impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        BSTIterator {
            stack: vec![],
            cur: root,
        }
    }

    fn next(&mut self) -> i32 {
        while self.cur.is_some() {
            let cur = self.cur.take().unwrap();
            let left = cur.borrow_mut().left.take();
            self.stack.push(cur);
            self.cur = left;
        }

        let cur = self.stack.pop().unwrap();
        let val = cur.borrow().val;
        self.cur = cur.borrow_mut().right.take();
        val
    }

    fn has_next(&self) -> bool {
        self.cur.is_some() || !self.stack.is_empty()
    }
}