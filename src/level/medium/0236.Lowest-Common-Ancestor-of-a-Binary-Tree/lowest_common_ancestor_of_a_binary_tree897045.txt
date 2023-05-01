// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/solutions/897045/rust-4ms-100/
use std::cell::RefCell;
use std::rc::Rc;

///
/// get "Pointer" of a Tree Node
fn to_rc(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        Some(ref node) => Some(Rc::clone(node)),
        None => None,
    }
}

///
/// get value of a Tree Node
fn val_of(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
    match root {
        Some(ref node) => {
            let node = node.borrow();
            Some(node.val)
        }
        None => None,
    }
}

///
/// get left of a Tree Node
fn left_of(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        Some(ref node) => {
            let node = node.borrow();
            match &node.left {
                Some(l) => Some(Rc::clone(l)),
                None => None,
            }
        }
        None => None,
    }
}

///
/// get right of a Tree Node
fn right_of(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        Some(ref node) => {
            let node = node.borrow();
            match &node.right {
                Some(r) => Some(Rc::clone(r)),
                None => None,
            }
        }
        None => None,
    }
}

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn recursive(
            curr: Option<Rc<RefCell<TreeNode>>>,
            p: Option<Rc<RefCell<TreeNode>>>,
            q: Option<Rc<RefCell<TreeNode>>>,
            ans: &mut Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            if curr.is_none() {
                return false;
            }
            let left = if recursive(left_of(&curr), to_rc(&p), to_rc(&q), ans) {
                1
            } else {
                0
            };
            let right = if recursive(right_of(&curr), to_rc(&p), to_rc(&q), ans) {
                1
            } else {
                0
            };
            let mid = if curr == p || curr == q { 1 } else { 0 };
            if mid + left + right >= 2 {
                *ans = curr;
            }
            left + mid + right > 0
        }
        let mut ans = None;
        recursive(root, p, q, &mut ans);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lowest_common_ancestor() {
        let left = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            }))),
        })));
        let right = Some(Rc::new(RefCell::new(TreeNode {
            val: 8,
            left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        })));
        let p = to_rc(&left);
        let q = to_rc(&right);
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 6,
            left,
            right,
        })));
        let expected = to_rc(&root);
        assert_eq!(Solution::lowest_common_ancestor(root, p, q), expected);
    }

    #[test]
    fn test_lowest_common_ancestor_02() {
        let lr = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        })));
        let q = to_rc(&lr);
        let left = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            right: lr,
        })));
        let p = to_rc(&left);
        let expected = to_rc(&left);
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 6,
            left,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 8,
                left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            }))),
        })));
        assert_eq!(Solution::lowest_common_ancestor(root, p, q), expected);
    }
}