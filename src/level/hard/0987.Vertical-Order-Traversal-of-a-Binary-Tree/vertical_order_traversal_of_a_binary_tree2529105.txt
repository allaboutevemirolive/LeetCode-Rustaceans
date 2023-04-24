// https://leetcode.com/problems/vertical-order-traversal-of-a-binary-tree/solutions/2529105/rust-btreemap-3ms-simple/
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

impl Solution {
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut map = BTreeMap::new();

        Self::vertical(root, (0, 0), &mut map);

        let mut vals: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
        for ((x, _), mut v) in map {
            v.sort_unstable();
            vals.entry(x).or_default().extend(v);
        }

        vals.into_values().collect()
    }

    pub fn vertical(
        root: Option<Rc<RefCell<TreeNode>>>,
        (x, y): (i32, i32),
        map: &mut BTreeMap<(i32, i32), Vec<i32>>,
    ) {
        if let Some(rc_node) = root {
            let mut node = rc_node.borrow_mut();
            map.entry((x, y)).or_default().push(node.val);
            let left = node.left.take();
            let right = node.right.take();
            Self::vertical(left, (x - 1, y + 1), map);
            Self::vertical(right, (x + 1, y + 1), map);
        }
    }
}
