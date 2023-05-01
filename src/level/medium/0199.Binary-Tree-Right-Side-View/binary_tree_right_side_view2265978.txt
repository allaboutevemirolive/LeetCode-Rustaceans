// https://leetcode.com/problems/binary-tree-right-side-view/solutions/2265978/rust-iterative-level-order-traversal/
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() { return Vec::new(); }
        let mut out: Vec<i32> = vec![];
        let mut this_row: Vec<Rc<RefCell<TreeNode>>> = vec![];
        let mut next_row: Vec<Rc<RefCell<TreeNode>>> = vec![];
        this_row.push( root.unwrap() );
        loop {
            if this_row.is_empty() { break; }
            let mut last = 0;
            for node in this_row.drain(..) {
                if let Some(L) = &node.borrow().left {
                    next_row.push( L.clone() );
                }
                if let Some(R) = &node.borrow().right {
                    next_row.push( R.clone() );
                }
                last = node.borrow().val.clone();
            }
            out.push(last.clone());
            this_row = next_row;
            next_row = vec![];
        }
        out
    }
}