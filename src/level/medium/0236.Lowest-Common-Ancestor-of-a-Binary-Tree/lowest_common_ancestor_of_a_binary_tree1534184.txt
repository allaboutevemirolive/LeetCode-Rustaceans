// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/solutions/1534184/rust-4-10ms/
use std::rc::Rc;
use std::cell::RefCell;

fn walk(root: Option<Rc<RefCell<TreeNode>>>, p_val: i32, q_val: i32) -> (Option<Rc<RefCell<TreeNode>>>, bool, bool) {
    match root {
        Some(node) => {
            let node_val = node.borrow().val;
            let mut p_seen = node_val == p_val;
            let mut q_seen = node_val == q_val;
    
            let right_result = walk(node.borrow().right.clone(), p_val, q_val);
            if !right_result.0.is_none() {
                return right_result;
            }
            p_seen = p_seen || right_result.1;
            q_seen = q_seen || right_result.2;
            
            if !(p_seen && q_seen) {
                let left_result = walk(node.borrow().left.clone(), p_val, q_val);
                if !left_result.0.is_none() {
                    return left_result;
                }
                p_seen = p_seen || left_result.1;
                q_seen = q_seen || left_result.2;
            }
            
            if p_seen && q_seen {
                (Some(node.clone()), true, true)
            } else {
                (None, p_seen, q_seen)
            }
        },
        None => {
            (None, false, false)
        }
    }
}

impl Solution {
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        walk(root, p.unwrap().borrow().val, q.unwrap().borrow().val).0
    }
}