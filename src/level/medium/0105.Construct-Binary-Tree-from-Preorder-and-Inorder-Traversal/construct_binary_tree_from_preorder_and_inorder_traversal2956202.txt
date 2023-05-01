// https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/solutions/2956202/rust-dfs-easy-to-understand/
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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        
        fn dfs(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            match preorder.first() {
                Some(root) => {
                    let root_index = inorder.iter().enumerate().find(|(index, val)| &root == val).unwrap().0;
                    return Some(Rc::new(RefCell::new(TreeNode { 
                        val: *root, 
                        left: dfs(&preorder[1..(root_index + 1)], &inorder[..root_index]), 
                        right: dfs(&preorder[(root_index+1)..], &inorder[(root_index+1)..])
                        })));
                },
                None => None
            }
        }
        dfs(&preorder, &inorder)
    }
}