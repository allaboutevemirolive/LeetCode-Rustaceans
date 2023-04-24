// https://leetcode.com/problems/binary-tree-cameras/solutions/2161389/rust-recursive/
use std::rc::Rc;
use std::cell::RefCell;
use Status::{Camera, Covered, NotCovered};

enum Status {
    Camera,
    Covered,
    NotCovered,
}

impl Solution {
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut camera_count = 0;

        if let NotCovered = dfs(&root, &mut camera_count) {
            camera_count += 1;
        }

        camera_count
    }
}

fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, camera_count: &mut i32) -> Status {
    match node {
        None => Covered,
        Some(node) => {
            let node = node.borrow();

            let left_status = dfs(&node.left, camera_count);
            let right_status = dfs(&node.right, camera_count);

            match (left_status, right_status) {
                (Covered, Covered) => NotCovered,
                (Camera, Camera) | (Camera, Covered) | (Covered, Camera) => Covered,
                (NotCovered, _) | (_, NotCovered) => {
                    *camera_count += 1;
                    Camera
                }
            }
        }
    }
}