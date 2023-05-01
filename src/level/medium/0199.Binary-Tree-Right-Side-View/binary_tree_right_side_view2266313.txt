// https://leetcode.com/problems/binary-tree-right-side-view/solutions/2266313/rust-iterative-bfs-no-clone-with-explanation/
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut rez = vec![];
        if root.is_none() {
            return rez;
        }
        let mut q = std::iter::once(root.unwrap()).collect::<VecDeque<_>>();

        let queue_children =
            |node_rc: Rc<RefCell<TreeNode>>, q: &mut VecDeque<Rc<RefCell<TreeNode>>>| {
                let mut node_ref = node_rc.borrow_mut();

                if let Some(left) = node_ref.left.take() {
                    q.push_back(left);
                }
                if let Some(right) = node_ref.right.take() {
                    q.push_back(right);
                }
            };

        while !q.is_empty() {
            for _ in 0..q.len() - 1 {
                queue_children(q.pop_front().unwrap(), &mut q);
            }

            let node_rc = q.pop_front().unwrap();
            rez.push(node_rc.borrow().val);
            queue_children(node_rc, &mut q);
        }
        rez
    }
}