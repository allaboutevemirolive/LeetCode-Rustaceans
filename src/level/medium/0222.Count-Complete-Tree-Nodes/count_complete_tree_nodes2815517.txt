// https://leetcode.com/problems/count-complete-tree-nodes/solutions/2815517/rust-binary-search-the-binary-tree/
use std::rc::Rc;
use std::cell::RefCell;
type OptNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn count_nodes(root: OptNode) -> i32 {
        let left_len = Self::side_depth(&root, true);
        let right_len = Self::side_depth(&root, false);
        if left_len == right_len {
            return (1 << left_len) - 1;
        }
        let mut lo = 0;
        let mut hi = (1 << right_len) - 1;
        while lo < hi {
            let mid = (lo + hi) >> 1;
            if Self::binary_tree_search(mid, 1 << (right_len - 1), &root) {
                lo = mid + 1;
            }
            else {
                hi = mid;
            }
        }
        (1 << right_len) - 1 + lo
    }

    fn side_depth(node: &OptNode, is_left: bool) -> i32 {
        match node.as_ref() {
            None => 0,
            Some(n) => 1 + if is_left {
                Self::side_depth(&n.borrow().left, is_left)
            }
            else {
                Self::side_depth(&n.borrow().right, is_left)
            }
        }
    }

    fn binary_tree_search(x: i32, bit: i32, node: &OptNode) -> bool {
        match node.as_ref() {
            None => false,
            Some(n) => if bit == 0 {
                true
            }
            else if x & bit == 0 {
                Self::binary_tree_search(x, bit >> 1, &n.borrow().left)
            }
            else {
                Self::binary_tree_search(x, bit >> 1, &n.borrow().right)
            }
        }
    }
}