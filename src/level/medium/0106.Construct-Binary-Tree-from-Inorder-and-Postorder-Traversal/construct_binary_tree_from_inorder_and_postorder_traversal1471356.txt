// https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/solutions/1471356/rust/
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
use std::collections::HashMap;

type InorderMap = HashMap<i32, usize>;
type TreeResult = Option<Rc<RefCell<TreeNode>>>;

struct TreeBuilder {
    map: InorderMap,
    postorder: Vec<i32>,
    inorder: Vec<i32>,
    post_idx: i32,
}

impl TreeBuilder {
    fn new(inorder: Vec<i32>, mut postorder: Vec<i32>) -> Self {
        let index = (postorder.len() - 1) as i32;

        Self {
            map: inorder
                .iter()
                .enumerate()
                .map(|(x, &v)| (v, x))
                .collect::<InorderMap>(),
            postorder,
            inorder,
            post_idx: index,
        }
    }

    fn build_tree(&mut self, start: i32, end: i32) -> TreeResult {
        if start > end {
            return None;
        }

        let root = self.postorder[self.post_idx as usize];
        let inorder_root_index = *self.map.get(&root).unwrap() as i32;
        let root_node = Rc::new(RefCell::new(TreeNode::new(root)));

        self.post_idx -= 1;

        {
            let mut bor = root_node.borrow_mut();

            bor.right = self.build_tree(inorder_root_index + 1, end);

            bor.left = self.build_tree(start, inorder_root_index - 1);
        }

        Some(root_node)
    }
}

impl Solution {
    pub fn build_tree(inorder: Vec<i32>, mut postorder: Vec<i32>) -> TreeResult {
        let size = (inorder.len() - 1) as i32;
        let mut bt = TreeBuilder::new(inorder, postorder);

        bt.build_tree(0, size)
    }
}