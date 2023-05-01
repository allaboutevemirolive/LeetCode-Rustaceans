// https://leetcode.com/problems/binary-search-tree-iterator/solutions/965750/rust-iterative/
use std::{cell::RefCell, rc::Rc};

struct BSTIterator {
    stack: Vec<(Rc<RefCell<TreeNode>>, bool)>,
}

impl BSTIterator {
    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        Self {
            stack: root.map(|a| vec![(a, false)]).unwrap_or(vec![]),
        }
    }

    /** @return the next smallest number */
    pub fn next(&mut self) -> i32 {
        while let Some((node, left_seen)) = self.stack.pop() {
            if left_seen {
                if let Some(ref r) = node.borrow().right {
                    self.stack.push((r.clone(), false));
                }
                return node.borrow().val;
            } else {
                self.stack.push((node.clone(), true));
                if let Some(ref l) = node.borrow().left {
                    self.stack.push((l.clone(), false));
                }
            }
        }
        panic!()
    }

    /** @return whether we have a next smallest number */
    pub fn has_next(&self) -> bool {
        self.stack.len() > 0
    }
}