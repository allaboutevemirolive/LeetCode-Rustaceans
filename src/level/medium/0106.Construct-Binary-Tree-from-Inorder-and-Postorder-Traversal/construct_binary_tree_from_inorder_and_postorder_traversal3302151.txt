// https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/solutions/3302151/rust-clean-solution/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn build_tree(mut inorder: Vec<i32>, mut postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(inorder: &[i32], postorder: &mut Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
            if inorder.is_empty() { return None }

            let val = postorder.pop().unwrap();
            let index = inorder.iter().position(|&v| v == val).unwrap();

            let left = &inorder[..index];
            let right = &inorder[index + 1..];
            
            let mut node = TreeNode::new(val);
            node.right = dfs(&right, postorder);
            node.left = dfs(&left, postorder);

            Some(Rc::new(RefCell::new(node)))
        }
        dfs(&inorder, &mut postorder)
    }
}