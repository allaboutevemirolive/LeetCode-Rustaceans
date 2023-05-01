// https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/solutions/2279403/rust-yars-yet-another-recursive-solution/
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let hash: HashMap<i32, usize> = inorder.iter().enumerate()
            .map(|(i, &x)| (x, i))
            .collect();
        Self::build_node(&preorder, &inorder, 0, &hash)
    }
	
    fn build_node(preorder: &[i32], inorder: &[i32], offset: usize, hash: &HashMap<i32, usize>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() {
            return None;
        }
        let pos = hash.get(&preorder[0]).unwrap() - offset;
        Some(Rc::new(RefCell::new(TreeNode {
            val: preorder[0],
            left: Self::build_node(&preorder[1..pos + 1], &inorder[..pos], offset, hash),
            right: Self::build_node(&preorder[pos + 1..], &inorder[pos + 1..], offset + pos + 1, hash),
        })))
    }
}