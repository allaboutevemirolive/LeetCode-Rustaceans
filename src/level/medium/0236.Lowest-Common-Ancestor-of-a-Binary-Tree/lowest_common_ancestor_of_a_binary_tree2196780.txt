// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/solutions/2196780/rust-recursive-o-n-o-1-with-comments/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    fn dfs(node_opt: Option<Rc<RefCell<TreeNode>>>, p: i32, q: i32) -> (bool, bool, Option<Rc<RefCell<TreeNode>>>) {
        match node_opt {
            None => (false, false, None),
            Some(node_rc) => {
                let mut node = node_rc.borrow_mut();
                // Recurse left branch to see if LCA is found there
                let (p_found_left, q_found_left, rez_left) = Self::dfs(node.left.take(), p, q);
                if rez_left.is_some() {
                    (true, true, rez_left)
                } else {
                    // LCA not found in left branch - search right instead
                    let (p_found_right, q_found_right, rez_right) = Self::dfs(node.right.take(), p, q);
                    if rez_right.is_some() {
                        (true, true, rez_right)
                    } else {
                        // LCA not found yet - check if this node can contribute
                        let p_found = p_found_left || p_found_right || node.val == p;
                        let q_found = q_found_left || q_found_right || node.val == q;
                        drop(node);
                        // If we now have found both p and q, this node is the LCA
                        let rez = if p_found && q_found { Some(node_rc) } else { None };
                        (p_found, q_found, rez)
                    }
                }
            }
        }
    }

    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let p = p.unwrap().borrow().val;
        let q = q.unwrap().borrow().val;

        Self::dfs(root, p, q).2
    }
}