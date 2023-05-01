// https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/solutions/3203456/rust/
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return Vec::new();
        }

        let mut q = Vec::new();
        let mut dir = 1;
        q.push(root);
        let mut ans = Vec::new();
        while !q.is_empty() {
            let mut nq = Vec::new();
            let mut t = Vec::new();
            for root in q.into_iter().rev() {
                if let Some(root) = root {
                    let mut root = root.borrow_mut();
                    t.push(root.val);
                    if dir == 0 {
                        nq.push(root.right.take());
                        nq.push(root.left.take());
                    } else {
                        nq.push(root.left.take());
                        nq.push(root.right.take());
                    }
                }
            }
            if !t.is_empty() {
                ans.push(t);
            }
            q = nq;
            dir = 1 - dir;
        }
        ans
    }
}