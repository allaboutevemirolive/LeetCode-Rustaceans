// https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/solutions/3204212/rust-solution/

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans : Vec<Vec<i32>> = vec![];

        let mut queue = VecDeque::new();

        queue.push_back(root);
        let mut i = 0;
        while queue.len() > 0 {
            let lt = queue.len();
            let mut res : Vec<i32> = vec![];

            for i in 0..lt {
                let a = queue.pop_front();
                
                if let Some(v) = a{
                    if let Some(nn) = v {
                        let n = nn.borrow();
                        res.push(n.val);
                        queue.push_back(n.left.clone());
                        queue.push_back(n.right.clone());
                    }
                    
                }
            }   

            if (i&1) == 1 {
                res.reverse();
            }

            if res.len() > 0 {
                ans.push(res);
            }
            i += 1;
        }
        ans
    }
}