// https://leetcode.com/problems/binary-tree-right-side-view/solutions/2269434/rust/
use std::{rc::Rc, cell::RefCell, collections::VecDeque, iter::{from_fn, once}};
impl Solution {
    fn bfs(mut root: Rc<RefCell<TreeNode>>) -> impl Iterator<Item = (Rc<RefCell<TreeNode>>, i32)> {
        let mut queue: VecDeque<_> = once((root, 1)).collect();
            
        from_fn(move || {
            let (node, level) = queue.pop_front()?;
            
            {
                let node = node.borrow();
                queue.extend(node.right.iter().chain(&node.left).map(|node| (node.clone(), level + 1)));
            }

            Some((node, level))
        })
    }
    
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        root.into_iter().flat_map(|node| {
            Self::bfs(node)
                .scan(0, |last_level, (node, level)| {
                    Some(if *last_level != level {
                        *last_level += 1;
                        Some(node.borrow().val)
                    } else {
                        None
                    })
            })
                .flatten()
        })
            .collect()
    }
}