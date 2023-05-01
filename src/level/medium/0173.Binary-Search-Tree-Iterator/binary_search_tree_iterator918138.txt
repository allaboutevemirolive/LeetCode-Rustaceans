// https://leetcode.com/problems/binary-search-tree-iterator/solutions/918138/rust-solution-please-review/
use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
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

struct BSTIterator {
    nodes: Vec<Rc<RefCell<TreeNode>>>,
}

impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut nodes = vec![];

        if let Some(root) = root {
            BSTIterator::insert(&mut nodes, root);
        }

        BSTIterator { nodes }
    }

    /// Return the next smallest number
    fn next(&mut self) -> i32 {
        let rc = self.nodes.pop().unwrap();
        let node = (*rc).borrow();

        if let Some(node) = &node.right {
            BSTIterator::insert(&mut self.nodes, Rc::clone(node));
        }

        node.val
    }

    /// True if there is a next smallest number, false otherwise
    fn has_next(&self) -> bool {
        !self.nodes.is_empty()
    }

    fn insert(nodes: &mut Vec<Rc<RefCell<TreeNode>>>, node: Rc<RefCell<TreeNode>>) {
        let mut node = Some(node);
        while let Some(rc) = node {
            nodes.push(Rc::clone(&rc));

            node = (*rc)
                .borrow()
                .left
                .as_ref()
                .and_then(|lft| Some(Rc::clone(&lft)));
        }
    }
}

fn main() {
    let mut node50 = TreeNode::new(50);
    let mut node25 = TreeNode::new(25);
    let mut node75 = TreeNode::new(75);
    let mut node12 = TreeNode::new(12);
    let mut node32 = TreeNode::new(32);
    let mut node62 = TreeNode::new(62);
    let mut node80 = TreeNode::new(80);

    node25.left = Some(Rc::new(RefCell::new(node12)));
    node25.right = Some(Rc::new(RefCell::new(node32)));

    node75.left = Some(Rc::new(RefCell::new(node62)));
    node75.right = Some(Rc::new(RefCell::new(node80)));

    node50.left = Some(Rc::new(RefCell::new(node25)));
    node50.right = Some(Rc::new(RefCell::new(node75)));

    let mut it = BSTIterator::new(Some(Rc::new(RefCell::new(node50))));

    while it.has_next() {
        println!("{}", it.next());
    }
}