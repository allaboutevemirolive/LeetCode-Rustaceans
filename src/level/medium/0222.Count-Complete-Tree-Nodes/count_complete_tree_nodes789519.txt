// https://leetcode.com/problems/count-complete-tree-nodes/solutions/789519/rust-recursive-solution/
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
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn countn(n: Option<Rc<RefCell<TreeNode>>>, c: &mut i32) {
            match n {
                Some(n) => {
                    *c += 1;
                    countn(n.borrow().left.clone(), c);
                    countn(n.borrow().right.clone(), c);
                }
                _ => ()
            }
        }
        let res = &mut 0i32;
        countn(root.clone(), res);
        *res
    }
}