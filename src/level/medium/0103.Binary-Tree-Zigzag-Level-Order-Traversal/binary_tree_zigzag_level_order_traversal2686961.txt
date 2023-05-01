// https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/solutions/2686961/rust-100/
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn zigzag_level_order(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        if root.is_none() {
            return res;
        }

        let mut cur_level = VecDeque::new();
        let mut next_level = VecDeque::new();

        cur_level.push_back(root.take().unwrap());
        let mut reverse = false;

        while !cur_level.is_empty() {
            let mut level = Vec::with_capacity(cur_level.len());
            while let Some(node) = cur_level.pop_front() {
                level.push(node.borrow().val);

                if let Some(left) = node.borrow_mut().left.take() {
                    next_level.push_back(left);
                }
                if let Some(right) = node.borrow_mut().right.take() {
                    next_level.push_back(right);
                }
            }

            if reverse {
                level.reverse();
            }
            res.push(level);
            reverse = !reverse;

            std::mem::swap(&mut cur_level, &mut next_level);
            next_level.clear();
        }
        res
    }
}