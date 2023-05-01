// https://leetcode.com/problems/sum-root-to-leaf-numbers/solutions/3294078/rust-simple-iterative-0ms/
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut stack  = vec![(0, root.unwrap())];
        let mut total  = 0;

        while let Some((sum, node)) = stack.pop() {
            let node = node.borrow();
            let sum  = sum * 10 + node.val;

            match (&node.left, &node.right) {
                (Some(left), Some(right)) => {
                    // Only the smart pointers get cloned, and not
                    // the nodes themselves.
                    stack.push((sum, left.clone()));
                    stack.push((sum, right.clone()));
                },
                (Some(child), None) |
                (None, Some(child)) => {
                    stack.push((sum, child.clone()));
                },
                (None, None) => {
                    total += sum;
                }
            }
        }
        total
    }
}