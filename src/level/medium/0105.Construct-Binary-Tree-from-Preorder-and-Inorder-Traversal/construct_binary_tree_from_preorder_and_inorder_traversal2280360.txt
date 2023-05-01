// https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/solutions/2280360/rust-recursive-map-approaches/
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
use std::collections::HashMap;

impl Solution {
    
    pub fn construct(preorder: &Vec<i32>, inorder: &Vec<i32>, mp: &HashMap<i32, i32>, preindex: &mut i32, in_begin: i32, in_end: i32) -> Option<Rc<RefCell<TreeNode>>> {
        
        if ( in_begin > in_end ) {
            return None
        }
        
        let cur = preorder[*preindex as usize];
        *preindex = *preindex + 1;
        
        let mut node = TreeNode::new(cur);
        
        if ( in_begin == in_end ) {
            return Some(Rc::new(RefCell::new(node)));
        }
        else {
            let in_index = mp[&cur];
            node.left = Solution::construct(preorder, inorder, mp, preindex, in_begin, in_index - 1);
            node.right = Solution::construct(preorder, inorder, mp, preindex, in_index + 1, in_end);
            
            return Some(Rc::new(RefCell::new(node)));
        }
        
    }
    
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        
        let mut mp = HashMap::<i32, i32>::new(); // (Node val, order index)
        
        
        for i in 0..inorder.len() as usize {
            mp.insert(inorder[i], i as i32);
        }
        
        let mut preindex = 0; 
        
        Solution::construct(&preorder, &inorder, &mp, &mut preindex, 0, (inorder.len() - 1) as i32)
        
    }
}