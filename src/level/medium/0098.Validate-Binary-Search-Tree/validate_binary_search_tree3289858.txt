// https://leetcode.com/problems/validate-binary-search-tree/solutions/3289858/rust-2-solutions/
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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn tree_props(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
            fn check(root: Option<Rc<RefCell<TreeNode>>>, lo: Option<i32>, hi: Option<i32>) -> bool {
                if let Some(r) = root {
                    let r = r.borrow();
                    if lo.is_some() && r.val <= lo.unwrap() {
                        return false;
                    }
                    if hi.is_some() && r.val > hi.unwrap() {
                        return false;
                    }
                    check(r.left.clone(), lo, Some(r.val)) && check(r.right.clone(), Some(r.val), hi)
                } else {
                    true
                }
            }
            check(root, None, None)
        }
        fn inorder_check(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
            fn inorder(root: Option<Rc<RefCell<TreeNode>>>, prev: &mut Option<i32>) -> bool {
                if let Some(r) = root {
                    let r = r.borrow();
                    if !inorder(r.left.clone(), prev) {
                        return false;
                    }
                    if prev.is_some() && prev.unwrap() >= r.val {
                        return false;
                    }
                    *prev = Some(r.val);
                    inorder(r.right.clone(), prev)
                } else {
                    true
                }
            }
            inorder(root, &mut None)
        }
        inorder_check(root)        
    }
}