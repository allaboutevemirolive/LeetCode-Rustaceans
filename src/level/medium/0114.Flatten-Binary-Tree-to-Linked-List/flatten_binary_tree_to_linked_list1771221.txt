// https://leetcode.com/problems/flatten-binary-tree-to-linked-list/solutions/1771221/rust-0ms-iterative-o-1-space/
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
        let mut node: Option<Rc<RefCell<TreeNode>>> = root.clone();

        while let Some(n) = node {
            let mut n = n.borrow_mut();
            if let Some(mut cur) = n.left.clone() {
                loop { // I couldn't figure out how to make this a while let...
                    let right = cur.borrow().right.clone();
                    if let Some(r) = right {
                        cur = r;
                    } else {
                        break;
                    }
                }
                let mut b = cur.borrow_mut();
                b.right = n.right.take();
                n.right = n.left.take();
            }

            node = n.right.clone();
        }
    }
}