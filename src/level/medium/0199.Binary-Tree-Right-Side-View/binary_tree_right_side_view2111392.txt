// https://leetcode.com/problems/binary-tree-right-side-view/solutions/2111392/rust-0ms-2mb/
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        let mut queue = VecDeque::new();
        
        queue.push_back((0, root));
        while let Some((lvl, node)) = queue.pop_front() { 
            if let Some(rc) = node { 
                let TreeNode {val, left, right} = Rc::try_unwrap(rc)
                    .unwrap().into_inner();
                /*
                    Once the right node has been considered,
                    the res.len() > lvl, we can then replace the 
                    current res[lvl] = [right node]
                */
                if lvl >= res.len() { 
                    res.push(val);
                } else { 
                    res[lvl] = val
                }
                
                queue.push_back((lvl + 1, left));
                queue.push_back((lvl + 1, right));
            } 
        }
        res
    }
}