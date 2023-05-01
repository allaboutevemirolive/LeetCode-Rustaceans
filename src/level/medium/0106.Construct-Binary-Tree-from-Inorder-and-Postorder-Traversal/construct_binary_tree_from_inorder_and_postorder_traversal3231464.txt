// https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/solutions/3231464/rust-easy-solution/
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
    pub fn solve(inorder: &mut Vec<i32>, postorder: &mut Vec<i32>,left : i32,right : i32,pleft : i32,pright : i32) -> Option<Rc<RefCell<TreeNode>>> {

        if right < left {
            return None;
        }

        let root = postorder[pright as usize];

        let mut m1 = 0;

        for x in left..right+1 {
            if inorder[x as usize] == root {
                m1 = x;
                break;
            }
        }

        let lleft = Self::solve(inorder,postorder,left,m1-1,pleft,pleft + m1-left-1);
        let rright = Self::solve(inorder,postorder,m1+1,right,pleft + m1-left,pright-1);
        
        let mut n = TreeNode::new(root);
        n.left = lleft;
        n.right = rright;

        return Some(Rc::new(RefCell::new(n)));
    }

    pub fn build_tree(mut inorder: Vec<i32>,mut postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let l = inorder.len()-1;
        let ans = Self::solve(&mut inorder,&mut postorder,0 as i32,l as i32,0 as i32,l as i32);
        ans
    }
}