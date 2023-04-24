// https://leetcode.com/problems/vertical-order-traversal-of-a-binary-tree/solutions/3162205/rust-binary-heap-recursive/
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        use std::cmp::*;
        use std::collections::*;
        type Node = Option<Rc<RefCell<TreeNode>>>;

        let mut heap = BinaryHeap::new();

        fn dfs(root: Node, coord: (i32, i32), heap: &mut BinaryHeap<Reverse<((i32, i32), i32)>>) {
            if root.is_none() {
                return;
            }
            let r = root.unwrap();
            let r = r.borrow();
            heap.push(Reverse(((coord.1, coord.0), r.val)));
            dfs(r.left.clone(), (coord.0 + 1, coord.1 - 1), heap);
            dfs(r.right.clone(), (coord.0 + 1, coord.1 + 1), heap);
        }

        dfs(root, (0, 0), &mut heap);
        let mut ans = vec![];
        let mut prev = i32::MIN;

        while let Some(Reverse(((lvl, _), val))) = heap.pop() {
            if lvl != prev {
                ans.push(vec![]);
            }
            ans.last_mut().unwrap().push(val);
            prev = lvl;
        }
        ans
    }
}