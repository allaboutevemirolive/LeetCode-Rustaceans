// https://leetcode.com/problems/binary-tree-level-order-traversal/solutions/2324047/simple-rust-bfs/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        let mut q = vec![root];
        while q.len() > 0{
            let mut level_values = vec![];
            let mut next_level = vec![];
            for node in q{
                if let Some(n) = node{
                    // get vals
                    level_values.push(n.borrow().val);
                    
                    // get next level of nodes
                    next_level.push(n.borrow().left.clone());
                    next_level.push(n.borrow().right.clone());
                }
            }
            
            // add level values and add children to queue
            if level_values.len() > 0{
                ret.push(level_values);
            }
            q = next_level;
        }
        ret
    }
}