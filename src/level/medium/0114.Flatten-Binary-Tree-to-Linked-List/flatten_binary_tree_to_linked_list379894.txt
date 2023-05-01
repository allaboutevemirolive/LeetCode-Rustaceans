// https://leetcode.com/problems/flatten-binary-tree-to-linked-list/solutions/379894/rust-0ms-postorder/
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
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(rt) = root {
            // cannot flatten a leaf node
            if rt.borrow().left.is_none() && rt.borrow().right.is_none() {
                return;
            }
            // flatten left
            Solution::flatten(&mut rt.borrow_mut().left);
            // flatten right
            Solution::flatten(&mut rt.borrow_mut().right);
            let mut node = rt.clone();
            if rt.borrow().left.is_none() {
                return;
            }
            let l = node.borrow().left.clone().unwrap();
            node = l;
            while !node.borrow().right.is_none() {
                let r = node.borrow().right.clone();
                node = r.unwrap();
            }
            let lft = rt.borrow().left.clone();
            (*node.borrow_mut()).right = rt.borrow().right.clone();
            (*rt.borrow_mut()).right = lft;
            (*rt.borrow_mut()).left = None;
        };
    }
}