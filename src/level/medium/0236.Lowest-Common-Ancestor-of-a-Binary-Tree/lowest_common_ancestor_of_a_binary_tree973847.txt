// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/solutions/973847/my-rust-solution/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, 
        q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        
            return dfs(root.as_ref(), p.as_ref(), q.as_ref());

            pub fn dfs(root : Option<&Rc<RefCell<TreeNode>>>,
                        p   : Option<&Rc<RefCell<TreeNode>>>, 
                        q   : Option<&Rc<RefCell<TreeNode>>>
            )-> Option<Rc<RefCell<TreeNode>>>{
                //3.
                if root == None || root == p || root == q {
                    return root.cloned();
                }


                //1. 
                let left = dfs(root.unwrap().borrow().left.as_ref(), p, q);
                let right = dfs(root.unwrap().borrow().right.as_ref(), p, q);
                //2.
                if left != None && right != None {
                    return root.cloned();
                }
                if left != None{
                    return left;
                }
                if right != None{
                    return right;
                }
                return None;
            }
    }
}