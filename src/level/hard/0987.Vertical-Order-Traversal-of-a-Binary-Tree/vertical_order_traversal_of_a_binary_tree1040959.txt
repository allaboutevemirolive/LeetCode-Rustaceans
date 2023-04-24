// https://leetcode.com/problems/vertical-order-traversal-of-a-binary-tree/solutions/1040959/rust-0ms-depth-first-iterative-recursive-solution/
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

impl Solution {
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut x_val_map = BTreeMap::<isize, BTreeMap<isize, Vec<_>>>::new();
        let mut stack = Vec::new();
        stack.push((root.clone(), 0, 0)); // start at the root; (0, 0)
		
		// visit all nodes, remembering the x & y positions of each node
        while let Some((node, x, y)) = stack.pop() {
            if let Some(node) = node {
                let node = node.borrow();
                let vals_at_xy = x_val_map.entry(x).or_default().entry(y).or_default();
                vals_at_xy.push(node.val);
                stack.push((node.left.clone(), x - 1, y + 1));
                stack.push((node.right.clone(), x + 1, y + 1));
            }
        }

        // sort node values that share the same x and y positions
        x_val_map
            .values_mut()
            .for_each(|m| m.values_mut().for_each(|v| v.sort_unstable()));
		// convert BTreeMap<_, BTreeMap<_, Vec<i32>>> to Vec<Vec<i32>>
        x_val_map
            .values()
            .map(|m| m.values().flatten().cloned().collect())
            .collect()
    }
}