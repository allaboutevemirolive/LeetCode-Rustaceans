// https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/solutions/3203561/rust-bfs-solution/
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = vec![];

        if root.is_none() {
            return res;
        }

        let mut deque: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        deque.push_back(root.unwrap());

        let mut is_left = true;

        while !deque.is_empty() {
            let size = deque.len();
            let mut level = vec![];

            for _i in 0..size {
                if let Some(node) = deque.pop_front() {
                    level.push(node.borrow().val);
                    
                    if let Some(left) = &node.borrow().left {
                        deque.push_back(Rc::clone(left));
                    }
                    if let Some(right) = &node.borrow().right {
                        deque.push_back(Rc::clone(right));
                    }
                }
            }
            
            if is_left {
                res.push(level) 
            } else {
                level.reverse();
                res.push(level);
            }

            is_left = !is_left;
        }
        
        res
    }
}