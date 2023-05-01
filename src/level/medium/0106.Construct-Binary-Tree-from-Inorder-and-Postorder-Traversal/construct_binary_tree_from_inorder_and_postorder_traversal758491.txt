// https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/solutions/758491/rust-recursive-solution/
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::helper(&inorder, &postorder)
    }
    fn helper(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(&last) = postorder.last() {
            let pos = inorder.iter().position(|&e| e == last).unwrap();
            Some(Rc::new(RefCell::new(TreeNode {
                val: last,
                left: Solution::helper(&inorder[0..pos], &postorder[0..pos]),
                right: Solution::helper(&inorder[pos + 1..], &postorder[pos..postorder.len() - 1]),
            })))
        } else {
            None
        }
    }
}