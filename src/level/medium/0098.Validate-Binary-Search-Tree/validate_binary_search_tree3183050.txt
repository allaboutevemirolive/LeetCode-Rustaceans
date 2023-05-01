// https://leetcode.com/problems/validate-binary-search-tree/solutions/3183050/rust-solution/
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
    pub fn solve(root : &Option<Rc<RefCell<TreeNode>>>) -> (i64,i64,bool) {
        if let Some(v) = root{
            let n = v.borrow();

            let (mnleft,mxleft,currleft)  = Self::solve(&n.left);
            let (mnright,mxright,currright)  = Self::solve(&n.right);

            if (n.val as i64) > mxleft && (n.val as i64) < mnright && currleft && currright {
                let mut mn : i64 = i64::MIN;
                let mut mx : i64 = i64::MAX;

                if mnleft == i64::MAX && mnright == i64::MAX{
                    mn = n.val as i64;
                    mx = n.val as i64;                
                } 
                else if mnleft == i64::MAX {
                    mn = n.val as i64;
                    mx = mxright;  
                }
                else if mnright == i64::MAX {
                    mn = mnleft;
                    mx = n.val as i64;  
                }
                else{
                    mn = mnleft;
                    mx = mxright;
                }
                return (mn,mx,true);
            }
            return (mnleft,mnright,false);

        }
        return (i64::MAX,i64::MIN,true);
    }
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // let mut mn : i32;
        // let mut mx : i32;

        let (mnleft,mnright,ans) = Self::solve(&root);

        ans
    }
}