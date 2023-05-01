// https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/solutions/3303343/rust-clean-easy-solution/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::builder(&inorder, &preorder)
    }
    pub fn builder(inorder: &[i32], preorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>>{
        if inorder.is_empty() {
            return None
        }

        let mut root = TreeNode::new(preorder[0]);

        let in_split = inorder.split(|&val| val == root.val).collect::<Vec<&[i32]>>();

        root.left = Self::builder(in_split[0], &preorder[1..=in_split[0].len()]);
        root.right = Self::builder(in_split[1], &preorder[in_split[0].len()+1..]);
        Some(Rc::new(RefCell::new(root)))
    }
}