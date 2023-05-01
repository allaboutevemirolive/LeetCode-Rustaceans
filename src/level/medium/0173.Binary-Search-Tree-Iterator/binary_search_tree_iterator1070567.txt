// https://leetcode.com/problems/binary-search-tree-iterator/solutions/1070567/rust-without-moving-the-original-tree-out/
use crate::leetcode::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

type MyNode = Rc<RefCell<TreeNode>>;

pub struct BSTIterator {
    stack: RefCell<Vec<MyNode>>,
}

impl BSTIterator {
    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut iter = None;
        if let Some(node) = root.as_ref() {
            iter = Some(Rc::clone(node));
        }

        let ret = BSTIterator {
            stack: RefCell::new(vec![]),
        };
        while let Some(node) = iter {
            ret.stack.borrow_mut().push(Rc::clone(&node));

            let val = node.borrow();
            match &val.left {
                Some(left) => iter = Some(Rc::clone(left)),
                None => {
                    break;
                }
            }
        }

        ret
    }

    pub fn next(&self) -> i32 {
        let top = self.stack.borrow_mut().pop().expect("stack is empty");

        let val = top.borrow().val;

        // build a new option with ref count incremented.
        let mut iter = match top.borrow().right.as_ref() {
            Some(val) => Some(val.clone()),
            None => return val,
        };

        while let Some(node) = iter {
            self.stack.borrow_mut().push(Rc::clone(&node));
            match node.borrow().left.as_ref() {
                Some(val) => iter = Some(Rc::clone(val)),
                None => break,
            }
        }

        val
    }

    pub fn has_next(&self) -> bool {
        !self.stack.borrow().is_empty()
    }
}