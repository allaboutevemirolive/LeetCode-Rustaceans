// https://leetcode.com/problems/count-complete-tree-nodes/solutions/701935/rust-recursion-simple/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(r) = root {
            return Solution::count_nodes(r.borrow().left.clone()) + 1 + Solution::count_nodes(r.borrow().right.clone());
        }
        else {
            return 0;
        } 
    }
}