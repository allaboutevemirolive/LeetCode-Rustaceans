// https://leetcode.com/problems/binary-tree-level-order-traversal/solutions/3335004/rust-deque/
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        if let Some(node) = root {
            let mut node_queue: VecDeque<(Rc<RefCell<TreeNode>>, usize)> = VecDeque::new();
            let mut level: Vec<i32> = Vec::new();
            node_queue.push_back((node, 0));
            while !node_queue.is_empty() {
                let (cur, l) = node_queue.pop_front().unwrap();
                if l > res.len() {
                    res.push(level);
                    level = Vec::<i32>::new();
                }
                level.push(cur.borrow().val);
                let l = l + 1;
                match (cur.borrow().left.clone(), cur.borrow().right.clone()) {
                    (Some(left_child), Some(right_child)) => {
                        node_queue.push_back((left_child, l));
                        node_queue.push_back((right_child, l));
                    },
                    (Some(child), None) | (None, Some(child)) => node_queue.push_back((child, l)),
                    _ => (),
                };
            }
            if level.len() > 0 {
                res.push(level);
            }
        }
        res
    }
}