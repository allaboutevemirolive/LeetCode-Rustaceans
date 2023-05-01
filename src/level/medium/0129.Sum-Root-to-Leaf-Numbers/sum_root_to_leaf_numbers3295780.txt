// https://leetcode.com/problems/sum-root-to-leaf-numbers/solutions/3295780/rust-dfs-solution/
use std::{cell::RefCell, rc::Rc};

impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, acc: i32, result: &mut i32) {
            if let Some(x) = node {
                let x = x.borrow();
                let acc = acc * 10 + x.val;
                if x.left.is_none() && x.right.is_none() {
                    *result += acc;
                } else {
                    dfs(x.left.clone(), acc, result);
                    dfs(x.right.clone(), acc, result);
                }
            }
        }
        let mut result = 0;
        dfs(root, 0, &mut result);
        result
    }
}