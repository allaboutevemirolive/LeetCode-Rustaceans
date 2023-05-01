// https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/solutions/2686816/rust-split/
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::helper(&preorder, &inorder)
    }

    fn helper(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() {
            return None;
        }
        let root = preorder.first().unwrap();
        let inorder_idx = inorder.iter().position(|x| x == root).unwrap();

        let (inorder_left, inorder_right) = inorder.split_at(inorder_idx);
        let (_, inorder_right) = inorder_right.split_first().unwrap();

        Some(Rc::new(RefCell::new(TreeNode {
            val: *root,
            left: Self::helper(&preorder[1..1 + inorder_left.len()], inorder_left),
            right: Self::helper(&preorder[1 + inorder_left.len()..], inorder_right),
        })))
    }
}