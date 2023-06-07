// https://leetcode.com/problems/binary-tree-cameras/solutions/2160421/rust-greedy-approach/
impl Solution {
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (count, dist) = Self::cal_count_dist(root.as_ref().unwrap());
        count + (dist >> 1)
    }

    fn cal_count_dist(node: &Rc<RefCell<TreeNode>>) -> (i32, i32) {
        let b = node.borrow();
        let (l_count, l_dist) = match b.left.as_ref() {
            Some(child) => Self::cal_count_dist(child),
            None => (0, 1),
        };
        let (r_count, r_dist) = match b.right.as_ref() {
            Some(child) => Self::cal_count_dist(child),
            None => (0, 1),
        };
        if l_dist == 2 || r_dist == 2 { // either child is not covered
            (l_count + r_count + 1, 0)
        }
        else {
            (l_count + r_count, l_dist.min(r_dist) + 1)
        }
    }
}