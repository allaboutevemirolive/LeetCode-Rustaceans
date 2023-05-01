// https://leetcode.com/problems/sum-root-to-leaf-numbers/solutions/3294014/rust/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;
        Self::foo(&root, 0, &mut sum);
        sum
    }

    fn foo(root: &Option<Rc<RefCell<TreeNode>>>, mut prefix: i32, sum: &mut i32) {
        if let Some(root) = root {
            let root = root.borrow();
            prefix = prefix * 10 + root.val;
            if root.left.is_none() && root.right.is_none() {
                *sum += prefix;
                return;
            }
            Self::foo(&root.left, prefix, sum);
            Self::foo(&root.right, prefix, sum);
        }
    }
}