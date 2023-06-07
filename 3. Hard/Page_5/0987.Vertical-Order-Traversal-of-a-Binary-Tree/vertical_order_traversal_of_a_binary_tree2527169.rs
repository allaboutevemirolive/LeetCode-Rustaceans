// https://leetcode.com/problems/vertical-order-traversal-of-a-binary-tree/solutions/2527169/rust-yars-yet-another-recursive-solution/
use std::rc::Rc;
use std::cell::RefCell;
type OptNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut left = Vec::new();
        let mut right = Vec::new();
        Self::dfs(&root, 0, 0, &mut left, &mut right);
        left.into_iter().rev().chain(right.into_iter())
        .map(|v2d| v2d.into_iter()
            .flat_map(|mut v| {
                v.sort_unstable();
                v
            }).collect()
        ).collect()
    }

    fn dfs<'a>(node: &OptNode, row: usize, col: i32,
    mut left: &'a mut Vec<Vec<Vec<i32>>>,
    mut right: &'a mut Vec<Vec<Vec<i32>>>) {
        if let Some(n) = node.as_ref() {
            let b = n.borrow();
            let side = if col >= 0 { &mut right } else { &mut left };
            let side_col = if col >= 0 { col } else { -col - 1 } as usize;
            if side.len() == side_col {
                side.push(Vec::new());
            }
            if side[side_col].len() <= row {
                side[side_col].resize(row + 1, Vec::new());
            }
            side[side_col][row].push(b.val);
            Self::dfs(&b.left, row + 1, col - 1, left, right);
            Self::dfs(&b.right, row + 1, col + 1, left, right);
        }
    }
}