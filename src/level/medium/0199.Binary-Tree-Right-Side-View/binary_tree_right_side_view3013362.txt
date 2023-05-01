// https://leetcode.com/problems/binary-tree-right-side-view/solutions/3013362/rust-0ms-dfs-using-level/

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        let mut have_seen = HashMap::new();
        Self::right_view_dfs(&root, &mut ans, &mut have_seen, 0);
        ans
    }

    pub fn right_view_dfs(
        node: &Option<Rc<RefCell<TreeNode>>>,
        right_vals: &mut Vec<i32>,
        have_seen: &mut HashMap<i32, bool>,
        level: i32,
    ) {
        if let Some(node) = node {
            let node = node.borrow();
            if have_seen.get(&level).is_none() {
                right_vals.push(node.val);
                have_seen.insert(level, true);
            }
            Self::right_view_dfs(&node.right, right_vals, have_seen, level + 1);
            Self::right_view_dfs(&node.left, right_vals, have_seen, level + 1);
        }
    }
}