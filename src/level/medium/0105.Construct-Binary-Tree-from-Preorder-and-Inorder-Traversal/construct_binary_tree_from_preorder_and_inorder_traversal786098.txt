// https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/solutions/786098/rust-0ms/
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::BTreeMap;
impl Solution {
    fn calc(preorder: &[i32], inorder: &[i32], i: &mut i32, j: i32, k: i32, table: &BTreeMap<i32, usize>) -> Option<Rc<RefCell<TreeNode>>> {
        if j > k {
            return None;
        }
        let mut node = TreeNode::new(preorder[*i as usize]);
        *i += 1;
        let p = table[&node.val] as i32;
        let left = Self::calc(preorder, inorder, i, j, p-1, table);
        let right = Self::calc(preorder, inorder, i, p+1, k, table);
        node.left = left;
        node.right = right;
        Some(Rc::new(RefCell::new(node)))
    }
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut i = 0;
        let table = inorder.iter().enumerate().map(|(p, &v)| (v, p)).collect::<BTreeMap<i32, usize>>();
        Self::calc(&preorder, &inorder, &mut i, 0, preorder.len() as i32 - 1, &table)
    }
}