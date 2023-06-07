// https://leetcode.com/problems/vertical-order-traversal-of-a-binary-tree/solutions/777825/rust-binaryheap-and-dfs/
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        use std::collections::BinaryHeap;

        fn helper(
            root: &Option<Rc<RefCell<TreeNode>>>,
            x: i32,
            y: i32,
            heap: &mut BinaryHeap<(i32, i32, i32)>,
        ) {
            if let Some(node) = root {
                let node = node.borrow();
                heap.push((x, y, -node.val));
                helper(&node.left, x - 1, y - 1, heap);
                helper(&node.right, x + 1, y - 1, heap);
            }
        }

        let mut heap = BinaryHeap::<(i32, i32, i32)>::new(); // x, y, val
        helper(&root, 0, 0, &mut heap);
        let mut ans = vec![];
        let mut x = std::i32::MIN;
        let mut v = vec![];
        while !heap.is_empty() {
            let h = heap.pop().unwrap();
            if h.0 != x {
                if !v.is_empty() {
                    ans.insert(0, v);
                }
                x = h.0;
                v = vec![];
            }
            v.push(-h.2);
        }
        if !v.is_empty() {
            ans.insert(0, v);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertical_traversal() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            }))),
        })));
        assert_eq!(
            Solution::vertical_traversal(root),
            vec![vec![9], vec![3, 15], vec![20], vec![7]]
        )
    }

    #[test]
    fn test_vertical_traversal_02() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            }))),
        })));
        assert_eq!(
            Solution::vertical_traversal(root),
            vec![vec![4], vec![2], vec![1, 5, 6], vec![3], vec![7]]
        )
    }
}