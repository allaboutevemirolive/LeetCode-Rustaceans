// https://leetcode.com/problems/flatten-binary-tree-to-linked-list/solutions/977238/rust-cheapest-best/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if root.is_some() {
            let x = std::mem::replace(root, None);
            let mut v = vec![];
            Self::destructure(x, &mut v);
    
            let dummy = Rc::new(RefCell::new(TreeNode::new(0)));
            let mut pointer = dummy.clone();
            while let Some(node) = v.pop() {
                if let Some(n) = node {
                    pointer.borrow_mut().right = Some(n.clone());
                    pointer = n;
                }
            }
    
            let _ = std::mem::replace(root, Some(dummy.borrow_mut().right.take().unwrap()));
        }
    }

    fn destructure(root: Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<Option<Rc<RefCell<TreeNode>>>>) {
        if let Some(node) = root {
            Self::destructure(node.borrow_mut().right.take(), v);
            Self::destructure(node.borrow_mut().left.take(), v);
            v.push(Some(node));
        }
    }
}