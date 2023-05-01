// https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/solutions/962765/rust-recursive/
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::helper(&preorder, &inorder)
    }

    fn helper(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root) = preorder.first() {
            let pivot_idx = inorder
                .iter()
                .enumerate()
                .find(|(_, v)| v == &root)
                .unwrap()
                .0;
            return Some(Rc::new(RefCell::new(TreeNode {
                val: *root,
                left: Self::helper(&preorder[1..(1 + pivot_idx)], &inorder[0..pivot_idx]),
                right: Self::helper(&preorder[(1 + pivot_idx)..], &inorder[(pivot_idx + 1)..]),
            })));
        } else {
            return None;
        }
    }
}