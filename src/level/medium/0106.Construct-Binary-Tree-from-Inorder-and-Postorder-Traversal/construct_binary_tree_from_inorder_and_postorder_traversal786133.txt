// https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/solutions/786133/rust-0ms/
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::BTreeMap;

impl Solution {
    fn calc(postorder: &[i32], inorder: &[i32], i: &mut i32, j: i32, k: i32, table: &BTreeMap<i32, usize>) -> Option<Rc<RefCell<TreeNode>>> {
        if j > k {
            return None;
        }
        let mut node = TreeNode::new(postorder[*i as usize]);
        *i -= 1;
        let p = table[&node.val] as i32;
        let right = Self::calc(postorder, inorder, i, p+1, k, table);
        let left = Self::calc(postorder, inorder, i, j, p-1, table);
        node.left = left;
        node.right = right;
        Some(Rc::new(RefCell::new(node)))
    }
    
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut i = postorder.len() as i32 - 1;
        let table = inorder.iter().enumerate().map(|(p, &v)| (v, p)).collect::<BTreeMap<i32, usize>>();
        Self::calc(&postorder, &inorder, &mut i, 0, inorder.len() as i32 - 1, &table)
    }
}