// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/solutions/2691070/rust-recursive-with-match/
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(ref root_rc) = root {
            let p_val = p.as_ref().unwrap().borrow().val;
            let q_val = q.as_ref().unwrap().borrow().val;

            if root_rc.borrow().val == p_val || root_rc.borrow().val == q_val {
                return root;
            }

            let left_lca = Self::lowest_common_ancestor(
                root_rc.borrow_mut().left.take(),
                p.clone(),
                q.clone(),
            );
            let right_lca = Self::lowest_common_ancestor(root_rc.borrow_mut().right.take(), p, q);

            return match (&left_lca, &right_lca) {
                (Some(_), Some(_)) => root,
                (Some(_), None) => left_lca,
                (None, Some(_)) => right_lca,
                (None, None) => None,
            };
        }
        None
    }
}