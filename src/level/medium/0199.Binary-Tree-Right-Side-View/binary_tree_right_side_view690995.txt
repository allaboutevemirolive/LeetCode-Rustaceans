// https://leetcode.com/problems/binary-tree-right-side-view/solutions/690995/rust-bfs/
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root {
            Some(tree) => {
                let mut result = vec![];
                let mut queue = VecDeque::new();
                queue.push_back((0, tree));
                while queue.len() > 0 {
                    let (level, node) = queue.pop_front().unwrap();
                    let n = node.borrow();
                    if result.len() == level {
                        result.push(n.val)
                    }
                    if n.right.is_some() {
                        queue.push_back((level + 1, n.right.clone().unwrap()));
                    }
                    if n.left.is_some() {
                        queue.push_back((level + 1, n.left.clone().unwrap()));
                    }
                }
                result
            },
            None => vec![]
        }
    }
}