// https://leetcode.com/problems/count-complete-tree-nodes/solutions/2193090/rust-solution-using-dfs/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
  pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(root) = root {
      dfs(&root)
    } else {
      0
    }
  }
}

fn dfs(node: &Rc<RefCell<TreeNode>>) -> i32 {
  let lv = if let Some(left) = &node.borrow().left {
    dfs(&left)
  } else {
    0
  };

  let rv = if let Some(right) = &node.borrow().right {
    dfs(&right)
  } else {
    0
  };

  1 + rv + lv
}