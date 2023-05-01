// https://leetcode.com/problems/binary-tree-level-order-traversal/solutions/3287977/rust-2-approaches/
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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
            fn dfs(root: Option<Rc<RefCell<TreeNode>>>, depth: usize, levels: &mut Vec<Vec<i32>>) {
                if let Some(r) = root {
                    if levels.len() == depth {
                        levels.push(vec![]);
                    }
                    let r = r.borrow();
                    levels[depth].push(r.val);
                    dfs(r.left.clone(), depth + 1, levels);
                    dfs(r.right.clone(), depth + 1, levels);
                }
            }
            let mut levels = vec![];
            dfs(root, 0, &mut levels);
            levels
        }
        
        fn bfs(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
            use std::collections::VecDeque;
            let mut levels = vec![];
            let mut queue = VecDeque::new();
            if root.is_some() {
                queue.push_back(root);
            }
            let mut lvl = 0;
            while !queue.is_empty() {
                let mut k = queue.len();
                if levels.len() == lvl {
                    levels.push(vec![]);
                }
                while k > 0 {
                    k -= 1;
                    if let Some(Some(n)) = queue.pop_front() {
                        let n = n.borrow();
                        levels[lvl].push(n.val);
                        if n.left.is_some() {
                            queue.push_back(n.left.clone());
                        }
                        if n.right.is_some() {
                            queue.push_back(n.right.clone());
                        }
                    }
                }
                lvl += 1;
            }
            levels
        }
        bfs(root)      
    }
}