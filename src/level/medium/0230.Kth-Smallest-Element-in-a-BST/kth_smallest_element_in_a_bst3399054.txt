// https://leetcode.com/problems/kth-smallest-element-in-a-bst/solutions/3399054/rust-implementation/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, order: &mut Vec<i32>) {
        if let Some(root_ref) = root {
            let root_node = root_ref.borrow();
            Self::dfs(&root_node.left, order);
            order.push(root_node.val);
            Self::dfs(&root_node.right, order);
        }
    }
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut order: Vec<i32> = Vec::new();
        Self::dfs(&root, &mut order);
        order[k as usize - 1]
    }
}