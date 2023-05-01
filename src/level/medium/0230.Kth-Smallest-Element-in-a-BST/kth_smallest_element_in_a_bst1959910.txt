// https://leetcode.com/problems/kth-smallest-element-in-a-bst/solutions/1959910/simple-rust-solution/
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut stack = vec![];
        Self::inorder(&root, &mut stack);

        stack[k as usize - 1]
    }

    fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, stack: &mut Vec<i32>) -> () {
        if let Some(node_rc) = node {
            let _node = node_rc.borrow();

            Self::inorder(&_node.left, stack);
            stack.push(_node.val);
            Self::inorder(&_node.right, stack);
        }
    }
}