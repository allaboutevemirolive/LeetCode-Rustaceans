// https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/solutions/1454238/rust-0ms/
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
impl Solution {
    fn construct_tree(preord: &Vec<i32>, inord: &Vec<i32>, index: &HashMap<i32, usize>, i: &mut usize, low: i32, high: i32) -> Option<Rc<RefCell<TreeNode>>>
    {
        if low <= high {
            let pos = *index.get(&preord[*i]).unwrap();
            let node = Rc::new(RefCell::new(TreeNode::new(inord[pos])));
            let pos = pos as i32;
            *i += 1;
            node.borrow_mut().left = Self::construct_tree(
                preord,
                inord,
                index,
                i,
                low,
                pos - 1
            );
            node.borrow_mut().right = Self::construct_tree(
                preord,
                inord,
                index,
                i,
                pos + 1,
                high
            );
            Some(node)
        } else {
            None
        }
    }
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let index = inorder.iter()
            .enumerate()
            .map(|(i, &v)| (v, i))
            .collect::<HashMap<i32, usize>>();
        let mut i = 0;
        let low = 0;
        let high = inorder.len() as i32 - 1;
        Self::construct_tree(
            &preorder,
            &inorder,
            &index,
            &mut i,
            low,
            high
        )
    }
}