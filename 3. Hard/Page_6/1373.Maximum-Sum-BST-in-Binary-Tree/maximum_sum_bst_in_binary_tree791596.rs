// https://leetcode.com/problems/maximum-sum-bst-in-binary-tree/solutions/791596/rust-translated-28ms-10-5m-100/
#[derive(Debug, Clone)]
pub struct Status {
    is_bst: bool,
    sum: i32,
    max_left: i32,
    min_right: i32,
}

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn max_sum_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn helper(root: &Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) -> Status {
            match root {
                None => Status {
                    is_bst: true,
                    sum: 0,
                    max_left: std::i32::MIN,
                    min_right: std::i32::MAX,
                },
                Some(node) => {
                    let node = node.borrow();
                    let left = helper(&node.left, v);
                    let right = helper(&node.right, v);
                    let val = node.val;
                    let s = if val > left.max_left && val < right.min_right {
                        Status {
                            is_bst: left.is_bst && right.is_bst,
                            sum: val + left.sum + right.sum,
                            max_left: val.max(right.max_left),
                            min_right: val.min(left.min_right),
                        }
                    } else {
                        Status {
                            is_bst: false,
                            sum: 0,
                            max_left: std::i32::MIN,
                            min_right: std::i32::MAX,
                        }
                    };
                    if s.is_bst {
                        v.push(s.sum)
                    }
                    s
                }
            }
        }
        let mut v = vec![];
        helper(&root, &mut v);
        v.iter().fold(0, |acc, &x| if x > acc { x } else { acc })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sum_bst() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                }))),
            }))),
        })));
        assert_eq!(Solution::max_sum_bst(root), 20)
    }

    #[test]
    fn test_max_sum_bst_02() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            }))),
            right: None,
        })));
        assert_eq!(Solution::max_sum_bst(root), 2)
    }

    #[test]
    fn test_max_sum_bst_03() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: -4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: -2,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(-5)))),
        })));
        assert_eq!(Solution::max_sum_bst(root), 0)
    }

    #[test]
    fn test_max_sum_bst_04() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        })));
        assert_eq!(Solution::max_sum_bst(root), 6)
    }

    #[test]
    fn test_max_sum_bst_05() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 10,
                left: Some(Rc::new(RefCell::new(TreeNode::new(-5)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(20)))),
            }))),
        })));
        assert_eq!(Solution::max_sum_bst(root), 25)
    }

    #[test]
    fn test_max_sum_bst_06() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 8,
                left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            }))),
        })));
        assert_eq!(Solution::max_sum_bst(root), 7)
    }
}