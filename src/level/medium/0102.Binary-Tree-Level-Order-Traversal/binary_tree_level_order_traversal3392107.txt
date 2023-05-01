// https://leetcode.com/problems/binary-tree-level-order-traversal/solutions/3392107/rust-version-of-bfs/
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
    pub fn level_order(root: Option<RcTreeNode>) -> Vec<Vec<i32>> {
       let mut queue: Vec<RcTreeNode> = vec![];
       let mut res: Vec<Vec<i32>> = vec![];

        if root.is_none() {
            return res;
        }

        queue.push(root.unwrap());

        while !queue.is_empty() {
            let len = queue.len();
            let mut arr: Vec<i32> = vec![];
            for i in 0..len {
                let head_node: RcTreeNode = queue.remove(0);
                let val = head_node.borrow().val;
                arr.push(val);
                if head_node.borrow().left.is_some() {
                    queue.push(head_node.borrow_mut().left.take().unwrap());
                }
                if head_node.borrow().right.is_some() {
                    queue.push(head_node.borrow_mut().right.take().unwrap());
                }
            }
            res.push(arr);
        }

        return res;
       
    }
}
