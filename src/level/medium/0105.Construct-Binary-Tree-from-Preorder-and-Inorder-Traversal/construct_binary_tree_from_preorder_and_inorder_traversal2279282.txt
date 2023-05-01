// https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/solutions/2279282/rust-recursively-consume-preorder/
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn build_tree(
        preorder: Vec<i32>,
        inorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // pre-order is left-to-right along each level
        // in-order is [*left, here, *right] (recursively)
        let mut po = VecDeque::from(preorder);
        build_subtree(&mut po, &inorder)
    }
}

fn build_subtree(
    preorder: &mut VecDeque<i32>,
    inorder: &[i32],
) -> Option<Rc<RefCell<TreeNode>>> {
    if inorder.is_empty() { return None; }
    let root: i32 = preorder.pop_front().unwrap();
    if inorder.len() == 1 {
        return Some(Rc::new(RefCell::new(TreeNode {
            val: root,
            left: None,
            right: None,
        })));
    }
    let pt: usize = inorder.iter()
        .enumerate()
        .find( |&(i,n)| *n == root )
        .unwrap().0;
    // Order matters due to `preorder` mutation
    let L = build_subtree(preorder, &inorder[ .. pt ] );
    let R = build_subtree(preorder, &inorder[ pt+1 .. ] );
    Some(Rc::new(RefCell::new(TreeNode {
        val: root,
        left: L,
        right: R,
    })))
}