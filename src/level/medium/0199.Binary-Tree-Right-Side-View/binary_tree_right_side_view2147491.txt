// https://leetcode.com/problems/binary-tree-right-side-view/solutions/2147491/rust-dfs/
pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut stack = vec![(root, 0)];
    let mut res = vec![];
    while let Some((node, level)) = stack.pop() {
        if let Some(rc_node) = node {
            if level as usize == res.len() {
                res.push(rc_node.borrow().val);
            };
            
            stack.push((rc_node.borrow().left.clone(), level+1));
            stack.push((rc_node.borrow().right.clone(), level+1));
        }
    }
    res
}