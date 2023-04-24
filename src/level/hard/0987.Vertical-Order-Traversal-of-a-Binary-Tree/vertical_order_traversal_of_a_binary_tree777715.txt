// https://leetcode.com/problems/vertical-order-traversal-of-a-binary-tree/solutions/777715/rust-dfs-solution/
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap};

impl Solution {
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut btm: BTreeMap<i32, BinaryHeap<Reverse<(i32, i32)>>> = BTreeMap::new();
        Solution::dfs(&root, (0, 0), &mut btm);
        let mut answer: Vec<Vec<i32>> = Vec::new();
        for bh in btm.values_mut() {
            let mut v: Vec<i32> = Vec::new();
            while let Some(r) = bh.pop() {
                v.push((r.0).1);
            }
            answer.push(v);
        }
        answer
    }
    fn dfs(
        node: &Option<Rc<RefCell<TreeNode>>>,
        pos: (i32, i32),
        btm: &mut BTreeMap<i32, BinaryHeap<Reverse<(i32, i32)>>>,
    ) {
        if let Some(n) = node {
            btm.entry(pos.0)
                .or_insert_with(BinaryHeap::new)
                .push(Reverse((pos.1, n.borrow().val)));
            Solution::dfs(&n.borrow().left, (pos.0 - 1, pos.1 + 1), btm);
            Solution::dfs(&n.borrow().right, (pos.0 + 1, pos.1 + 1), btm);
        }
    }
}