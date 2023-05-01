// https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/solutions/1548714/rust-recursive-with-hashmap/
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    fn make_tree(
        preorder: &mut &[i32],
        inorder: &[i32],
        inorder_positions: &HashMap<i32, usize>,
        inorder_offset: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() || inorder.is_empty() {
            return None;
        }
        let root = preorder[0];
        *preorder = &preorder[1..];
        let root_pos = inorder_positions[&root] - inorder_offset;
        let left = Self::make_tree(
            preorder,
            &inorder[..root_pos],
            inorder_positions,
            inorder_offset,
        );
        let right = Self::make_tree(
            preorder,
            &inorder[(root_pos + 1)..],
            inorder_positions,
            inorder_offset + root_pos + 1,
        );
        Some(Rc::new(RefCell::new(TreeNode {
            val: root,
            left,
            right,
        })))
    }
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let inorder_positions = inorder
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, x)| (x, i))
            .collect();
        Self::make_tree(&mut &preorder[..], &inorder, &inorder_positions, 0)
    }
}