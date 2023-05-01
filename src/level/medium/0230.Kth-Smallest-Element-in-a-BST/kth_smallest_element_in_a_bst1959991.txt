// https://leetcode.com/problems/kth-smallest-element-in-a-bst/solutions/1959991/rust-recursive-and-iterative-without-clone/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        Self::inorder(root, k, 0).1.unwrap()
    }

    fn inorder(node_option: Option<Rc<RefCell<TreeNode>>>, k: i32, acc: i32) -> (i32, Option<i32>) {
        if let Some(node_rc) = node_option {
            let mut node_ref = node_rc.borrow_mut();
            let (acc, r) = Self::inorder(node_ref.left.take(), k, acc);
            if acc == k {
                return (acc, r);
            }
            let acc = acc + 1;
            if acc == k {
                return (acc, Some(node_ref.val));
            }
            Self::inorder(node_ref.right.take(), k, acc)
        } else {
            (acc, None)
        }
    }
}