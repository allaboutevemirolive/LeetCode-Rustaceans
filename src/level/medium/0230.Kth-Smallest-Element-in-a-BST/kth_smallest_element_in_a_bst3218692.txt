// https://leetcode.com/problems/kth-smallest-element-in-a-bst/solutions/3218692/rust-easy-solution/
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
    pub fn solve(root: &Option<Rc<RefCell<TreeNode>>>,ans : &mut Vec<i32>) {

        if let Some(v) = root {
            let n = v.borrow();

            Self::solve(&n.left,ans);
            ans.push(n.val);
            Self::solve(&n.right,ans);
        }
    }

    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut ans : Vec<i32> = vec![];

        Self::solve(&root,&mut ans);
        ans[k as usize - 1]
    }
}