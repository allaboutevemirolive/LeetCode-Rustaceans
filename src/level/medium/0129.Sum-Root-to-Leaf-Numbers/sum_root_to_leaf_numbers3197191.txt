// https://leetcode.com/problems/sum-root-to-leaf-numbers/solutions/3197191/rust-preorder-solution/
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
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn preorder(
            root: Option<Rc<RefCell<TreeNode>>>,
            mut current_sum: i32,
            sum_leaf_numbers: &mut i32,
        ) {
            if let Some(r) = root {
                let r = r.borrow();
                current_sum *= 10;
                current_sum += r.val;
                if r.left.is_none() && r.right.is_none() {
                    *sum_leaf_numbers += current_sum;
                }
                preorder(r.left.clone(), current_sum, sum_leaf_numbers);
                preorder(r.right.clone(), current_sum, sum_leaf_numbers);
            }
        }
        let mut ans = 0;
        preorder(root, 0, &mut ans);
        ans     
    }
}