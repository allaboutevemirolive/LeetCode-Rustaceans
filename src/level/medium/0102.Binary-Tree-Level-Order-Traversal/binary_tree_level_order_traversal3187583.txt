// https://leetcode.com/problems/binary-tree-level-order-traversal/solutions/3187583/painful-rust/
use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution {}

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        if let Some(node) = root {
            let mut queue = VecDeque::new();
            queue.push_back(node);
            while !queue.is_empty() {
                let level_size = queue.len();
                let mut level = Vec::with_capacity(level_size);
                for _ in 0..level_size {
                    if let Some(node) = queue.pop_front() {
                        let node = node.borrow();
                        level.push(node.val);
                        if let Some(left) = node.left.clone() {
                            queue.push_back(left);
                        }
                        if let Some(right) = node.right.clone() {
                            queue.push_back(right);
                        }
                    }
                }
                result.push(level);
            }
        }
        result
    }
}

fn main() {
    let root = Rc::new(RefCell::new(TreeNode::new(3)));
    let left = Rc::new(RefCell::new(TreeNode::new(9)));
    let right = Rc::new(RefCell::new(TreeNode {
        val: 20,
        left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
    }));
    root.borrow_mut().left = Some(left);
    root.borrow_mut().right = Some(right);

    let result = Solution::level_order(Some(root));
    println!("{:?}", result);
}
