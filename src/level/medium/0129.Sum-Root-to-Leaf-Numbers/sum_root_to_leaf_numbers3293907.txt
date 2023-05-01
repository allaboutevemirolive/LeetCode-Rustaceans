// https://leetcode.com/problems/sum-root-to-leaf-numbers/solutions/3293907/rust-depth-first-solution/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, s: i32, res: &mut i32) {
            match node {
                None => (),
                Some(node) => {
                    let node = node.borrow();
                    let ns = s * 10 + node.val;
                    if node.left.is_none() && node.right.is_none() {
                        *res += ns;
                        return
                    }

                    dfs(&node.left, ns, res);
                    dfs(&node.right, ns, res);
                }
            };
        }

        let mut res = 0;
        dfs(&root, 0, &mut res);
        res   
    }
}