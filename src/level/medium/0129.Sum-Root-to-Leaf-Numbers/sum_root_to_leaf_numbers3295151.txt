// https://leetcode.com/problems/sum-root-to-leaf-numbers/solutions/3295151/rust-solution-dfs/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = vec![];
        Self::dfs(&root, 0, &mut res);
        res.iter().sum::<i32>()
    }
    pub fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, mut path: i32, res: &mut Vec<i32>) {
        match node {
            Some(treenode) => {
                let node = treenode.borrow();
                path = path*10 +  node.val;
                if node.left.is_none() && node.right.is_none() {
                    return res.push(path)
                }
                Self::dfs(&node.right, path, res);
                Self::dfs(&node.left, path, res);
            }
            None => {}
        }
    }
}