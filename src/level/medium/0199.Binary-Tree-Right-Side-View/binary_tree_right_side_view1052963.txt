// https://leetcode.com/problems/binary-tree-right-side-view/solutions/1052963/python-rust-bfs-queue/

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let node = match root {
            Some(a) => a,
            None => return vec![],
        };
        
        let mut res = vec![];
        let mut queue = VecDeque::new();
        queue.push_back(node);
        
        while queue.len() > 0 {
            let end = queue.len() - 1;
            res.push( queue[end].borrow().val.clone() );
            
            for _ in 0..=end {
                let node = queue.pop_front().unwrap();
                if let Some(ref left) = node.clone().borrow().left {
                    queue.push_back(left.clone());
                }
                if let Some(ref right) = node.clone().borrow().right {
                    queue.push_back(right.clone());
                }
            }
        }
        
        res
    }
}