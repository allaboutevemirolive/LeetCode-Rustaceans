// https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/solutions/2349296/rust-bfs/
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

use std::collections::VecDeque;

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        // bfs
        // maintain a next node queue
        let mut queue = VecDeque::new();
        let mut result = Vec::new();
        let mut is_reverse = false;
        queue.push_back(vec![root.clone()]);

        while let Some(layer) = queue.pop_front() {
            let mut next_layer = Vec::new();
            let mut curr_layer = Vec::new();
            for node in layer {
                if let Some(node) = node.as_ref() {
                    curr_layer.push(node.borrow().val);
                    if is_reverse {
                        if let Some(next) = node.borrow().right.clone() {
                            next_layer.push(Some(next));
                        }
                        if let Some(next) = node.borrow().left.clone() {
                            next_layer.push(Some(next));
                        }
                    } else {
                        if let Some(next) = node.borrow().left.clone() {
                            next_layer.push(Some(next));
                        }
                        if let Some(next) = node.borrow().right.clone() {
                            next_layer.push(Some(next));
                        }
                    }
                }
            };

            is_reverse = !is_reverse;
            next_layer.reverse();

            if next_layer.len()>0 {
                queue.push_back(next_layer);
            }
            if curr_layer.len()>0 {
                result.push(curr_layer);
            }
        }
        result
    }
}