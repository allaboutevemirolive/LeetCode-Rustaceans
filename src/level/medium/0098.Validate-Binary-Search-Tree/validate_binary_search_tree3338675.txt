// https://leetcode.com/problems/validate-binary-search-tree/solutions/3338675/rust-recursion/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    fn is_valid_node(root: Option<Rc<RefCell<TreeNode>>>, min_val: Option<i32>, max_val: Option<i32>) -> bool {
        if let Some(node) = root {
            let node = node.borrow();
            let valid = match (min_val, max_val) {
                (Some(min), Some(max)) => node.val > min && node.val < max,
                (Some(min), None) => node.val > min,
                (None, Some(max)) => node.val < max,
                _ => true,
            };
            valid &&
                Self::is_valid_node(node.left.clone(), min_val, Some(node.val)) &&
                Self::is_valid_node(node.right.clone(), Some(node.val), max_val)
        } else {
            true
        }
    }

    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_valid_node(root, None, None)
    }
}