// https://leetcode.com/problems/flatten-binary-tree-to-linked-list/solutions/2338798/rust-yars-yet-another-recursive-solution/
use std::rc::Rc;
use std::cell::RefCell;
type OptNode = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn flatten(root: &mut OptNode) {
        *root = Self::flat(root.take(), None);
    }
    
    pub fn flat(node: OptNode, after: OptNode) -> OptNode {
        match node {
            None => after,
            Some(n) => {
                let mut b = n.borrow_mut();
                let right = Self::flat(b.right.take(), after);
                b.right = Self::flat(b.left.take(), right);
                drop(b);
                Some(n)
            }
        }
    }
}