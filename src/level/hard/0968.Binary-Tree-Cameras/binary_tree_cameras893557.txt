// https://leetcode.com/problems/binary-tree-cameras/solutions/893557/rust-translated-0ms-100/
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn solve(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
            if let Some(node) = root {
                let node = node.borrow();
                let left = solve(&node.left);
                let right = solve(&node.right);
                let min_left = std::cmp::min(left[1], left[2]);
                let min_right = std::cmp::min(right[1], right[2]);
                let d0 = left[1] + right[1];
                let d1 = std::cmp::min(left[2] + min_right, right[2] + min_left);
                let d2 = 1 + std::cmp::min(left[0], min_left) + std::cmp::min(right[0], min_right);
                vec![d0, d1, d2]
            } else {
                vec![0, 0, std::i32::MAX / 10]
            }
        }

        let ans = solve(&root);
        std::cmp::min(ans[1], ans[2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_camera_cover() {
        assert_eq!(
            Solution::min_camera_cover(Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(0))))
            })))),
            1
        );
    }

    #[test]
    fn test_min_camera_cover_02() {
        assert_eq!(
            Solution::min_camera_cover(Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode::new(0))))
                    }))),
                    right: None
                }))),
                right: None
            })))),
            2
        );
    }
}