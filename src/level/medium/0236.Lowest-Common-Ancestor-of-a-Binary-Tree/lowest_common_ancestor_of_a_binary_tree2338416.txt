// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/solutions/2338416/rust-0ms-100/
use std::rc::Rc;
use std::cell::RefCell;
type Node = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn lowest_common_ancestor(root: Node, p: Node, q: Node) -> Node {
        if root.is_none() {return None;}
        if root==p || root==q {return root;}
        let rt=root.clone();
        let l=Solution::lowest_common_ancestor(rt.unwrap().as_ref().borrow().left.clone(),p.clone(),q.clone());
        if l.is_some() {
            let rt=root.clone();
            let r=Solution::lowest_common_ancestor(rt.unwrap().as_ref().borrow().right.clone(),p,q);
            if r.is_some() {root} else {l} 
        } else {Solution::lowest_common_ancestor(root.unwrap().as_ref().borrow().right.clone(),p,q)}
    }
}