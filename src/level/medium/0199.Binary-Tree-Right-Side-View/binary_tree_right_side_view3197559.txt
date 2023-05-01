// https://leetcode.com/problems/binary-tree-right-side-view/solutions/3197559/rust-preorder-inverted-solution/
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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn preorder_inverted(
            root: Option<Rc<RefCell<TreeNode>>>,
            depth: i32,
            path: &mut Vec<i32>,
            max_depth: &mut i32,
        ) {
            if let Some(r) = root {
                let r = r.borrow();
                if *max_depth < depth {
                    *max_depth = depth;
                    path.push(r.val);
                }
                preorder_inverted(r.right.clone(), depth + 1, path, max_depth);
                preorder_inverted(r.left.clone(), depth + 1, path, max_depth);
            }
        }
        let mut max_depth = -1;
        let mut ans = vec![];
        preorder_inverted(root, 0, &mut ans, &mut max_depth);
        ans        
    }
}