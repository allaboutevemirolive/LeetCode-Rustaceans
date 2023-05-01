// https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/solutions/3205881/rust-bfs-clean-code/
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::iter::FromIterator; // for VecDeque::from_iter(root)

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut q = VecDeque::from_iter(root);
        let mut is_ltr = true;
        let mut ans = vec![];

        while !q.is_empty() {
            let len = q.len();
            let mut level_nodes = Vec::with_capacity(len);
            for _ in 0..len {
                let node_rc = q.pop_front().unwrap();
                let mut node = node_rc.borrow_mut();
                level_nodes.push(node.val);
                let children = node.left.take().into_iter().chain(
                    node.right.take()
                );
                q.extend(children);
                // or
                // q.extend(node.left.take())
                // q.extend(node.right.take())
            }
            if !is_ltr {
                level_nodes.reverse();
            }
            is_ltr = !is_ltr;
            ans.push(level_nodes);
        }

        ans
    }
}