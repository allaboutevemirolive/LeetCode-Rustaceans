// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/solutions/1316937/rust-recursive/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    fn helper(root: Option<&Rc<RefCell<TreeNode>>>, p: &Option<Rc<RefCell<TreeNode>>>, q: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(node) => {
                let left = Self::helper(node.borrow().left.as_ref(), &p, &q);
                let right = Self::helper(node.borrow().right.as_ref(), &p, &q);
                if let Some(l) = left.clone() {
                    if let Some(r) = right.clone() {
                        return Some(node.clone());
                    }
                }
                if let None = left {
                    if node.clone() == p.clone().unwrap() || node.clone() == q.clone().unwrap() {
                        return Some(node.clone());
                    } else {
                        return right;
                    }
                } else if let None = right {
                    if node.clone() == p.clone().unwrap() || node.clone() == q.clone().unwrap() {
                        return Some(node.clone());
                    } else {
                        return left;
                    }
                }
                None
            },
        }
    }
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::helper(root.as_ref(), &p, &q)
    }
}