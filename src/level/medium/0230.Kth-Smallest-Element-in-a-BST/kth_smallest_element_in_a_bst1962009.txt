// https://leetcode.com/problems/kth-smallest-element-in-a-bst/solutions/1962009/rust-0ms-3-1-mb/
use std::rc::Rc;
use std::cell::RefCell;
type Node = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn kth_smallest(root: Node, k: i32) -> i32 {
        let mut res = vec![];
        //  WE need the nodes to be in the right order 
        //  hence we Used Inorder
        inorder(root.clone(), &mut res);
        fn inorder(root: Node, res: &mut Vec<i32>) { 
            if let Some(node) = root { 
                let node = node.borrow();
                let val = node.val;
                inorder(node.left.clone(), res);
                res.push(val);
                inorder(node.right.clone(), res);           
            }
        }
        //  The nodes are already ordered there is no need to sort it 
        return res[k as usize -1]
    }
}