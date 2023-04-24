// https://leetcode.com/problems/binary-tree-cameras/solutions/2160935/rust-solution/
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

#[derive(Debug, PartialEq, Eq)]
enum NodeState {
    NoCover = 0,
    NoCamUnderMonitor = 1,
    CamUnderMonitor = 2,
}

impl Solution {
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        let state = Self::dfs(&root, &mut res);
        match state {
            NodeState::NoCover => res + 1,
            _ => res,
        }
    }
    
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, res: &mut i32) -> NodeState {
        if let Some(node) = root {
            let node = node.borrow();
            if node.left.is_none() && node.right.is_none() {
                return NodeState::NoCover;
            }
            let left_state = Self::dfs(&node.left, res);
            let right_state = Self::dfs(&node.right, res);
            match (left_state, right_state) {
                (NodeState::NoCover, _) | (_, NodeState::NoCover) => {
                    *res += 1;
                    NodeState::CamUnderMonitor
                }
                (NodeState::CamUnderMonitor, _) | (_, NodeState::CamUnderMonitor) => {
                    NodeState::NoCamUnderMonitor
                }
                (_, _) => NodeState::NoCover,
            }
        } else {
            NodeState::NoCamUnderMonitor
        }
    }
}