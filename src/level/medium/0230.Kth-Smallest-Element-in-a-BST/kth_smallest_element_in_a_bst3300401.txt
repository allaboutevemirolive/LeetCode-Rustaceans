// https://leetcode.com/problems/kth-smallest-element-in-a-bst/solutions/3300401/rust-dfs-solution/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut res = 0;
        Self::dfs(root.as_ref().unwrap(), k, &mut 0, &mut res);
        res
    }

      pub fn dfs(node: &Rc<RefCell<TreeNode>>, target: i32, curent_k: &mut i32, res: &mut i32) {
        if *curent_k == target {
            return
        }
        let node =  node.as_ref().borrow();
        if node.left.is_some() {
            Self::dfs(node.left.as_ref().unwrap(), target , curent_k, res)
        }
        *curent_k += 1;
        if *curent_k == target {
            *res = node.val;
        }
        
        if *curent_k <= target && node.right.is_some() {
            Self::dfs(node.right.as_ref().unwrap(), target , curent_k, res)
        }
    }
}