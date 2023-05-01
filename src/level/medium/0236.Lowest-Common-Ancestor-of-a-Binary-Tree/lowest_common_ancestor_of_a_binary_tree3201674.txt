// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/solutions/3201674/rust-solution/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn solve(root:&Option<Rc<RefCell<TreeNode>>>,p : i32,q : i32) -> Option<Rc<RefCell<TreeNode>>> {

        if let Some(v) = root {
            let n = v.borrow();
            
            if n.val == p || n.val == q {
                return Some(v.clone());
            }

            let left = Self::solve(&n.left,p,q);
            let right = Self::solve(&n.right,p,q);

            if left.is_some() && right.is_some() {
                return Some(v.clone());
            }

            if left.is_some(){
                return left;
            }

            if right.is_some(){
                return right;
            }
        }
        return None;
    }
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::solve(&root,p.unwrap().borrow().val,q.unwrap().borrow().val)
    }
}