// https://leetcode.com/problems/flatten-binary-tree-to-linked-list/solutions/1193762/rust-clean-solution-space-o-n-and-o-1/
pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, vec: &mut Vec<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = root {
            vec.push(node.clone());
            dfs(&node.borrow().left, vec);
            dfs(&node.borrow().right, vec);
        }
    }

    if root.is_none() {
        return;
    }
    let mut vec = vec![];
    dfs(&root, &mut vec);
    let mut curr = vec[0].clone();
    for i in 1..vec.len() {
        curr.borrow_mut().right = Some(vec[i].clone());
        curr.borrow_mut().left = None;
        curr = vec[i].clone();
    }
}