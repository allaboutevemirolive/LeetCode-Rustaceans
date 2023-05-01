// https://leetcode.com/problems/count-complete-tree-nodes/solutions/1064734/a-very-clear-rust-solution/
	pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        return match root {
            None => 0,
            Some(root) => {
                1 + Solution::count_nodes(root.borrow().left.clone())
                    + Solution::count_nodes(root.borrow().right.clone())
            }
        };
    }