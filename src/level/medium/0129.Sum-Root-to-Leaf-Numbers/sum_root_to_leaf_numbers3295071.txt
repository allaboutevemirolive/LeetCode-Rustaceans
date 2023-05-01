// https://leetcode.com/problems/sum-root-to-leaf-numbers/solutions/3295071/rust-simple-recursive-0ms/
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    fn sum_numbers_helper(node: Option<Rc<RefCell<TreeNode>>>, path_sum: i32, sum: &mut i32) {
        match node {
            None => (),
            Some(node) => {
                let curr = node.as_ref().borrow();
                let curr_sum = path_sum * 10 + curr.val;
                if curr.left.is_none() && curr.right.is_none() {
                    // leaf node - 
                    // terminate recursion for this path here
                    *sum += curr_sum;
                    return;
                }
                Self::sum_numbers_helper(curr.left.clone(), curr_sum, sum);
                Self::sum_numbers_helper(curr.right.clone(), curr_sum, sum);
            }
        }
    }
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum: i32 = 0;
        Self::sum_numbers_helper(root, 0, &mut sum);
        sum
    }
}