// https://leetcode.com/problems/vertical-order-traversal-of-a-binary-tree/solutions/2527790/rust-iterative-dfs-with-comments/
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::iter::once;
impl Solution {
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut stack = vec![(root, 0, 0)];
        let mut map = HashMap::<i32, Vec<(i32, i32)>>::new();
        let (mut min, mut max) = (0, 0);

        while let Some(tuple) = stack.pop() {
            if let (Some(node_rc), x, y) = tuple {
                min = min.min(x);
                max = max.max(x);
                let mut node_ref = node_rc.borrow_mut();
                map.entry(x).or_default().push((y, node_ref.val));
                stack.extend(once(node_ref.left.take()).map(|left| (left, x - 1, y + 1)).chain(once(node_ref.right.take()).map(|right| (right, x + 1, y + 1))));
            }
        }

        let mut rez = vec![vec![]; (max - min + 1) as usize];

        for i in min..=max {
            let mut row = map.get_mut(&i).unwrap();
            row.sort_unstable();
            rez[(i - min) as usize] = row.iter().map(|(_, val)| *val).collect();
        }
        
        rez
    }
}