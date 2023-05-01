// https://leetcode.com/problems/validate-binary-search-tree/solutions/2670321/rust-0ms-3mb/
use std::rc::Rc;
use std::cell::RefCell;
type Node = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn is_valid_bst(root: Node) -> bool {
        return Solution::is_valid(&root, &None, &None);
    }
    fn is_valid(root: &Node, left: &Node, right: &Node) -> bool { 
        if let Some(node) = root { 
            let node = node.borrow();
            if left.is_some() && left.as_ref().unwrap().borrow().val >= node.val || 
                right.is_some() && right.as_ref().unwrap().borrow().val <= node.val { 
                    return false; 
            }
            return Self::is_valid(&node.left, &left, &root) && 
                    Self::is_valid(&node.right, &root, &right);
            
        } else { 
            return true
        }
    }
}