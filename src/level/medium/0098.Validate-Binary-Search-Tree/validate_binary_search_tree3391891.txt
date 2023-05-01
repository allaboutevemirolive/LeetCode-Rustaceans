// https://leetcode.com/problems/validate-binary-search-tree/solutions/3391891/rust-in-order-traverse/
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
type RcTreeNode = Rc<RefCell<TreeNode>>;


impl Solution {
    pub fn is_valid_bst(root: Option<RcTreeNode>) -> bool {
        let mut res: (Option<i32>, bool) = (None, true);
        Self::helper(&root, &mut res);
        return res.1;
    }

    fn helper(node: &Option<RcTreeNode>, res: &mut (Option<i32>, bool)) {
        if node.is_none() {
            return;
        }

        let mut curval = node.as_ref().unwrap().borrow().val;
        let left: &Option<RcTreeNode> = &node.as_ref().unwrap().borrow().left;
        let right: &Option<RcTreeNode> = &node.as_ref().unwrap().borrow().right;

        Self::helper(left, res);
        match res.0 {
            Some(v) if curval <= v => {
                res.1 = false;
            },
            _ => {},
        }
        if res.1 == false {
            return;
        }
        res.0 = Some(curval);
        Self::helper(right, res);
    }
}