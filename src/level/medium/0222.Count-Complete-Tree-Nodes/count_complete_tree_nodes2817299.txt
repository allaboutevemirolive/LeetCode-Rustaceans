// https://leetcode.com/problems/count-complete-tree-nodes/solutions/2817299/rust-3ms/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(v) => Self::search(&v),
            None    => 0,
        }
    }

    pub fn search(root: &Rc<RefCell<TreeNode>>) -> i32 {
        1 + match &root.borrow().left {
            Some(v) => Self::search(&v),
            None    => 0,
        } + match &root.borrow().right {
            Some(v) => Self::search(&v),
            None    => 0,
        }
    }
}