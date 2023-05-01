// https://leetcode.com/problems/binary-tree-right-side-view/solutions/1052300/rust-bfs/
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let root = match root {
            Some(a) => a,
            None => return vec![],
        };

        // NOTE: bfs by level, find the last node on each level

        let mut res = vec![];
        let mut queue = VecDeque::new();
        queue.push_back(root);
        while queue.len() > 0 {
            let bound = queue.len() - 1;
            for i in 0..=bound {
                let node = queue.pop_front().unwrap();
                if i == bound {
                    res.push(node.borrow().val.clone());
                }
                if let Some(ref l) = node.clone().borrow().left {
                    queue.push_back(l.clone());
                }
                if let Some(ref r) = node.clone().borrow().right {
                    queue.push_back(r.clone());
                }
            }
        }
        res
    }
}