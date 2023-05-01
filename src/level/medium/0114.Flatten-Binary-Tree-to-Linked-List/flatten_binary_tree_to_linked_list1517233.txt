// https://leetcode.com/problems/flatten-binary-tree-to-linked-list/solutions/1517233/rust-in-place-0-4ms/
use std::rc::Rc;
use std::cell::RefCell;

fn _append_deep_right(root: Option<Rc<RefCell<TreeNode>>>, leaf: Option<Rc<RefCell<TreeNode>>>) {
    if !leaf.is_none() {
        match root {
            Some(root_node) => {
                if !root_node.borrow().right.is_none() {
                    _append_deep_right(root_node.borrow_mut().right.clone(), leaf);
                } else {
                    root_node.borrow_mut().right = leaf;
                }
            },
            None => {}
        }
    }
}

fn _flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    match root {
        Some(node) => {
            _flatten(&mut node.borrow_mut().right);
            if !node.borrow().left.is_none() {
                _flatten(&mut node.borrow_mut().left);
                let head = node.borrow_mut().left.take();
                let tail = node.borrow_mut().right.take();
                _append_deep_right(head.clone(), tail);
                _append_deep_right(Some(node.clone()), head);
            }
        },
        None => {}
    }
}

impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        _flatten(root);
    }
}