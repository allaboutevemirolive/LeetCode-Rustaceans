// https://leetcode.com/problems/flatten-binary-tree-to-linked-list/solutions/240147/solution-in-rust/
#[allow(dead_code)]
pub fn flatten(root: &Option<Rc<RefCell<TreeNode>>>) {
    fn process(root: Rc<RefCell<TreeNode>>) -> Rc<RefCell<TreeNode>> {
        {
            let mut node = root.borrow_mut();
            if node.left.is_some() {
                let lr = process(Rc::clone(node.left.as_ref().unwrap()));
                // println!("process: node:{:?}, lr: {:?}\n", node, lr);
                lr.borrow_mut().right = node.right.clone();
                node.right = node.left.clone();
                node.left = None;
                // println!("after process: node:{:?}, lr: {:?}\n", node, lr);
            }

            if node.right.is_some() {
                return process(Rc::clone(node.right.as_ref().unwrap()))
            }
        }
        return root
    }
    if let Some(node) = root { process(Rc::clone(&node)); };
}
