// https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/solutions/2192226/rust-recursive-no-hashmap/
impl Solution {
    fn build(inorder: &mut Vec<i32>, postorder: &mut Vec<i32>, bound: Option<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.is_empty() || (bound.is_some() && *inorder.last().unwrap() == bound.unwrap()) {
            return None;
        }
        let mut root = TreeNode::new(postorder.pop().unwrap());
        root.right = Self::build(inorder, postorder, Some(root.val));
        inorder.pop();
        root.left = Self::build(inorder, postorder, bound);
        Some(Rc::new(RefCell::new(root)))
    }

    pub fn build_tree(mut inorder: Vec<i32>, mut postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build(&mut inorder, &mut postorder, None)
    }
}