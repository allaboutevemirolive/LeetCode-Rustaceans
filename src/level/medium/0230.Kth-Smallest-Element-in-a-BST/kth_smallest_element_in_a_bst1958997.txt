// https://leetcode.com/problems/kth-smallest-element-in-a-bst/solutions/1958997/rust-yars-yet-another-recursive-solution/
pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    Self::ans_or_count(&root, k).0.unwrap()
}

fn ans_or_count(node: &Option<Rc<RefCell<TreeNode>>>, k: i32) -> (Option<i32>, i32) {
    match node {
        None => (None, 0),
        Some(n) => {
            let borrow = n.borrow();
            let left_count = match Self::ans_or_count(&borrow.left, k) {
                (Some(ans), _) => return (Some(ans), k),
                (None, c) if c == k - 1 => return (Some(borrow.val), k),
                (None, c) => c,
            };
            match Self::ans_or_count(&borrow.right, k - left_count - 1) {
                (Some(ans), _) => (Some(ans), k),
                (None, right_count) => (None, left_count + 1 + right_count),
            }
        }
    }
}