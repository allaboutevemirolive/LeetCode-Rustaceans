// https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/solutions/3304667/rust-recursive-split/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            postorder.last().map(|val| {
                let index = inorder.iter().position(|x| val == x).unwrap();
                Rc::new(RefCell::new(TreeNode {
                    val: *val,
                    left: build(&inorder[..index], &postorder[..index]),
                    right: build(&inorder[index + 1..], &postorder[index..postorder.len() - 1])
                }))
            })
        }
        build(&inorder, &postorder)
    }
}