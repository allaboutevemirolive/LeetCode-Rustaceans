// https://leetcode.com/problems/count-complete-tree-nodes/solutions/2817643/rust-binary-search-method/
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
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut count = 0;
        let mut depth = 0;
        // let mut node = Some(Rc::clone(root.as_ref().unwrap()));
        let mut node = match root {
            None => None,
            Some(_) => Some(Rc::clone(root.as_ref().unwrap())),
        };
        while let Some(current_node) = node {
            // node = (&(Rc::clone(&node.as_ref().unwrap()).borrow()).left.as_ref();
            // println!("{}", &(current_node.borrow()).val);
            node = (&(current_node.borrow()).left).as_ref().map(Rc::clone);
            depth += 1;
        }
        depth -= 1;
        if depth == -1  {return 0;}
        if depth ==  0  {return 1;}
        
        count = 1;
        let mut subtree = Some(Rc::clone(root.as_ref().unwrap()));
        
        while depth != 0 {
            println!("{}", &(Rc::clone(subtree
                                .as_ref()
                                .unwrap()).borrow()).val);
            node = Some(Rc::clone(subtree.as_ref().unwrap()));
            node = (&(Rc::clone(node
                                .as_ref()
                                .unwrap())
                            .borrow())
                        .left).as_ref().map(Rc::clone);
            let mut i = depth - 1;
            while i != 0 {
                node = (&(Rc::clone(node
                                .as_ref()
                                .unwrap())
                            .borrow())
                        .right).as_ref().map(Rc::clone);
                i -= 1;
            }
            
            if let None = node {
                subtree = (&(Rc::clone(subtree
                                .as_ref()
                                .unwrap())
                            .borrow())
                        .left).as_ref().map(Rc::clone);
                count = count * 2;
            } else {
                subtree = (&(Rc::clone(subtree
                                .as_ref()
                                .unwrap())
                            .borrow())
                        .right).as_ref().map(Rc::clone);
                count = count * 2 + 1;
            }
            depth -= 1;
            
            if let None = subtree {
                count -= 1;
            }
        }
        
        count
    }
}