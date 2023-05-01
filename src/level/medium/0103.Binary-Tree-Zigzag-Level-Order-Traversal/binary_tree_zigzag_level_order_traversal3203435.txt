// https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/solutions/3203435/rust-dfs-solution/
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
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        use std::collections::*;

        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, levels: &mut Vec<VecDeque<i32>>, depth: usize) {
            if let Some(r) = root {
                if levels.len() == depth {
                    levels.push(VecDeque::new())
                }
                let r = r.borrow();
                if depth % 2 == 0 {
                    levels[depth].push_back(r.val);
                } else {
                    levels[depth].push_front(r.val);
                }
                dfs(r.left.clone(), levels, depth + 1);
                dfs(r.right.clone(), levels, depth + 1);
            }
        }

        let mut ans = vec![];
        dfs(root, &mut ans, 0);
        ans.into_iter()
            .map(|x| x.into_iter().collect::<Vec<_>>())
            .collect()        
    }
}