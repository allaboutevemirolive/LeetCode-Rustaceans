// https://leetcode.com/problems/sum-root-to-leaf-numbers/solutions/3295621/rust-recursion/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    fn root_to_leaf(pre_val: i32, root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            let root = root.borrow();
            let curr_val = root.val + pre_val;
            let left_leaf = root.left.clone();
            let right_leaf = root.right.clone();
            match (left_leaf.is_some(), right_leaf.is_some()) {
                (true, true) => return Solution::root_to_leaf(curr_val * 10, left_leaf)
                    + Solution::root_to_leaf(curr_val * 10, right_leaf),
                (true, false) => return Solution::root_to_leaf(curr_val * 10, left_leaf),
                (false, true) => return Solution::root_to_leaf(curr_val * 10, right_leaf),
                (false, false) => return curr_val,
            }
        } else {
            return pre_val;
        }
    }
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::root_to_leaf(0, root)
    }
}