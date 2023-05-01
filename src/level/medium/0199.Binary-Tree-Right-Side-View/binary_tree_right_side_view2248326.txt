// https://leetcode.com/problems/binary-tree-right-side-view/solutions/2248326/rust-bfs/
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none(){
            return vec![];
        }
        
        let root = root.unwrap();
        let mut q = VecDeque::from([root]);
        let mut res = vec![];
        
        while !q.is_empty(){
            let len = q.len();
            
            for i in 0..q.len(){
                let node = q.pop_front().unwrap();
                let node = node.borrow();
                
                if i + 1 == len{
                    res.push(node.val);
                }
                
                if let Some(l) = node.left.clone(){
                    q.push_back(l);
                }
                
                if let Some(r) = node.right.clone(){
                    q.push_back(r);
                }
            }
        }
        
        res
    }
}