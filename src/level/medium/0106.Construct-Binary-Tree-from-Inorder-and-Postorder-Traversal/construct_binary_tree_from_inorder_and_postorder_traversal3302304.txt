// https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/solutions/3302304/rust-recursion/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(&root_val) = postorder.last() {
            let mut root_node = TreeNode::new(root_val);
            let root_indice = inorder.iter().position(|&x| x == root_val).unwrap();
            root_node.left =
                Self::build_tree(
                    inorder[..root_indice].to_vec(),
                    postorder[..root_indice].to_vec()
                );
            root_node.right =
                Self::build_tree(
                    inorder[root_indice+1..].to_vec(),
                    postorder[root_indice..postorder.len()-1].to_vec()
                );
            return Some(Rc::new(RefCell::new(root_node)))
        } else {
            return None
        }
    }
}