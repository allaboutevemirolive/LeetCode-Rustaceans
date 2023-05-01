// https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/solutions/3302295/rust/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build(&inorder, &postorder)
    }

    fn build(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.len() == 1 {
            return Some(Rc::new(RefCell::new(TreeNode::new(inorder[0]))));
        } else if inorder.len() == 0 {
            return None;
        }

        let root_val = *postorder.last().unwrap();
        let mut p = 0;
        while inorder[p] != root_val {
            p += 1;
        }

        Some(Rc::new(RefCell::new(TreeNode {
            val: root_val,
            left: Self::build(&inorder[..p], &postorder[..p]),
            right: Self::build(&inorder[p + 1..], &postorder[p..inorder.len() - 1])
        })))
    }
}