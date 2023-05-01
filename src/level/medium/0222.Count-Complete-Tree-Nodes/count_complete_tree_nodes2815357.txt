// https://leetcode.com/problems/count-complete-tree-nodes/solutions/2815357/brief-explanation-rust-solution/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // Check the root node
        Self::count_current(&root)
    }

    fn count_current(curr_node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0_i32;

        // Check if node exists
        if let Some(cell) = curr_node {
            // Borrow a reference from the RefCell 
            // Rc allows using its Item methods directly
            let node = cell.borrow();

            // Count current node
            ans += 1;

            // Check left child node and its childs (recursive call)
            ans += Self::count_current(&node.left);

            // Check right child node and its childs (recursive call)
            ans += Self::count_current(&node.right);
        }
        ans
    }
}