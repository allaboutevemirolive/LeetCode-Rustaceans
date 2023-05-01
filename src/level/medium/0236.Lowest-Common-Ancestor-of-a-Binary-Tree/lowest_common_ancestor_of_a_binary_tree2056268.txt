// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/solutions/2056268/rust-post-order-traverse/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root{
            None => None,
            Some(node) =>{
                if Some(node.clone()) == p || Some(node.clone()) == q {
                    return Some(node.clone());
                }
                
                let left = Self::lowest_common_ancestor(node.borrow().left.clone(), p.clone(), q.clone());
                let right = Self::lowest_common_ancestor(node.borrow().right.clone(), p.clone(), q.clone());
                
                // both left and right are not None
                if left.is_some() && right.is_some(){
                    return Some(node.clone());
                }
				
                // both left and right are None
                if left.is_none() && right.is_none(){
                    return None;
                }
                
                // one of left and right is None
                if left.is_none(){
                    return right;
                }else {
                    return left;
                }
            }
        }
    }
    
}