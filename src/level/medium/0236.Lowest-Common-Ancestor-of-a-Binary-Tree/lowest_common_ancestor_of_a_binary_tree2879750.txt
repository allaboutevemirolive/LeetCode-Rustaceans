// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/solutions/2879750/rust-yars/
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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() || root == p || root == q {
            return root;
        }

        let left = Solution::lowest_common_ancestor(
            root.clone().unwrap().borrow().left.clone(),
            p.clone(),
            q.clone(),
        );
        let right = Solution::lowest_common_ancestor(
            root.clone().unwrap().borrow().right.clone(),
            p,
            q,
        );

        if left.is_some() && right.is_some() {
            return root;
        }

        if left.is_some() {
            left
        } else {
            right
        }
    }
}