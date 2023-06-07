// https://leetcode.com/problems/vertical-order-traversal-of-a-binary-tree/solutions/777916/rust-dfs-and-bfs-0ms/
use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

    impl Solution {
        pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
            let mut result = BTreeMap::<i32, Vec<i32>>::new();
            let mut queue = Vec::<(Option<Rc<RefCell<TreeNode>>>, i32)>::new();

            queue.push((root, 0));

            while !queue.is_empty() {
                let mut new_queue = Vec::new();
                let mut level = BTreeMap::<i32, Vec<i32>>::new();

                for (current, pos) in queue {
                    if let Some(node) = current {
                        level.entry(pos).or_default().push(node.borrow().val);
                        new_queue.push((node.borrow().left.clone(), pos - 1));
                        new_queue.push((node.borrow().right.clone(), pos + 1));
                    }
                }

                for (k, mut v) in level {
                    v.sort();
                    result.entry(k).or_default().extend(v);
                }

                queue = new_queue;
            }

            result.into_iter().map(|(_, value)| value).collect()
        }
    }