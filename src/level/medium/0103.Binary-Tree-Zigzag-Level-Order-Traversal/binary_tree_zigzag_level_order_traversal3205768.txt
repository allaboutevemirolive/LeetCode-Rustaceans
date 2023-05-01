// https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/solutions/3205768/rust-100/
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{VecDeque};
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        
        let mut r = vec![];
        let mut q = VecDeque::from([root.clone()]);
        let mut do_reverse = false;

        while q.len() > 0 {
            let mut rr = vec![];
            
            for _ in 0..q.len() {
                if let Some(Some(node)) = q.pop_front() {
                    let TreeNode {left, right, val} = &mut *node.borrow_mut();
                    q.push_back(left.clone());
                    q.push_back(right.clone());
                    rr.push(val.clone());
                }
            }

            if do_reverse {
                rr.reverse();
            }

            if rr.len() > 0 {
                r.push(rr);
            }

            do_reverse = if do_reverse { false } else { true };
        }

        r
    }
}










