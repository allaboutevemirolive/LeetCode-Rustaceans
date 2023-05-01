// https://leetcode.com/problems/count-complete-tree-nodes/solutions/2819660/rust-custom-type-trait-for-pointer-option/
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn count_nodes(root: NodeOpt) -> i32 {
        root.total_nodes()
    }
}

/// Type defined to reduce clutter in lines of code.
/// 
pub type NodeOpt = Option<Rc<RefCell<TreeNode>>>;

/// Trait defined to add methods to the custom type.
/// 
pub trait NodeOptTrait {
    fn new(val: i32, left: NodeOpt, right: NodeOpt) -> Self;
    fn left_height(&self) -> i32;
    fn right_height(&self) -> i32;
    fn total_nodes(&self) -> i32;
}

impl NodeOptTrait for NodeOpt {
    /// Creates a new node.
    /// 
    fn new(val: i32, left: NodeOpt, right: NodeOpt) -> Self {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }

    /// Returns the height of the node's subtree following each
    /// left child. If the node is `None`, 0 is returned.
    /// 
    fn left_height(&self) -> i32 {
        let mut height = 0;
        let mut node_opt = self.clone();

        while let Some(node) = node_opt {
            height += 1;
            node_opt = node.borrow().left.clone();
        }
        height
    }

    /// Returns the height of the node's subtree following each right 
    /// child. If the node is `None`, 0 is returned.
    /// 
    fn right_height(&self) -> i32 {
        let mut height = 0;
        let mut node_opt = self.clone();

        while let Some(node) = node_opt {
            height += 1;
            node_opt = node.borrow().right.clone();
        }
        height
    }

    /// Returns the total number of nodes in the tree.
    /// 
    fn total_nodes(&self) -> i32 {
        if let Some(node) = self {
            let h_left = node.borrow().left.left_height() + 1;
            let h_right = node.borrow().right.right_height() + 1;

            if h_left == h_right {
                2_i32.pow(h_left as u32) - 1
            } else {
                node.borrow().left.total_nodes() + 
                node.borrow().right.total_nodes() + 1
            }
        } else {
            0
        }
    }
}