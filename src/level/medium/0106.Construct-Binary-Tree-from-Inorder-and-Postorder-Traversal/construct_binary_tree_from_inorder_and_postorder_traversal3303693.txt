// https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/solutions/3303693/rust-intiutive/
pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let root_element = postorder.last()?;
        // println!("root {root_element} in{inorder:#?} post{postorder:#?}");
        let root_idx_in_inorder = inorder
            .iter()
            .position(|e| e == root_element)
            .expect("root element not found");
        let left_inorder = &inorder[..root_idx_in_inorder];
        let right_inorder = &inorder[root_idx_in_inorder + 1..];

        let left = if !left_inorder.is_empty() {
            let left_postorder = &postorder[..left_inorder.len()];
            // println!("left in{left_inorder:#?} post{left_postorder:#?}");
            Solution::build_tree(left_inorder.to_vec(), left_postorder.to_vec())
        } else {
            None
        };
        let right = if !right_inorder.is_empty() {
            // dbg!(left_inorder.len());
            let right_postorder = &postorder[left_inorder.len()..(postorder.len() - 1)];
            // println!("right in{right_inorder:#?} post{right_postorder:#?}");
            Solution::build_tree(right_inorder.to_vec(), right_postorder.to_vec())
        } else {
            None
        };
        Some(Rc::new(RefCell::new(TreeNode {
            val: root_element.to_owned(),
            left,
            right,
        })))
}