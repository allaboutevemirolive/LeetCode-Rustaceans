// https://leetcode.com/problems/kth-smallest-element-in-a-bst/solutions/1961890/rust-simple-iterative-solution-0ms/
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut stack = Vec::with_capacity(256);
        let mut node  = root.clone();
        let mut count = 0;
        let mut value = -1;

        'outer: while node.is_some() {
            // Descend left.
            while let Some(n) = node {
                node = n.borrow().left.clone();
                stack.push(n);
            }
            // Ascend and increment count.
            while let Some(n) = stack.pop() {
                let pnode  = n.borrow();
                    value  = pnode.val;
                    count += 1;
                // Check done.
                if count >= k { 
                    break 'outer; 
                }
                // If right child, prepare to descend left again.
                if pnode.right.is_some() {
                    node = pnode.right.clone();
                    break;
                }
            }
        }
        value
    }
}