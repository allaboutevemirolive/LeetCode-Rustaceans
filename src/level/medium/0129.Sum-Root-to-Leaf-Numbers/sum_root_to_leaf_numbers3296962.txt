// https://leetcode.com/problems/sum-root-to-leaf-numbers/solutions/3296962/rust-copy-and-study/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        Self::traverse(root.unwrap(), 0, &mut ans);
        ans
    }

    fn traverse(node: Rc<RefCell<TreeNode>>, mut cur: i32, ans: &mut i32) {
        cur = cur * 10 + node.borrow().val;
        if is_leaf(node.clone()) {
            return *ans += cur;
        }
        
        if let Some(ref v) = node.borrow().left {
            Self::traverse(v.clone(), cur, ans);
        }

        if let Some(ref v) = node.borrow().right {
            Self::traverse(v.clone(), cur, ans);
        }
    }
}

fn is_leaf(node: Rc<RefCell<TreeNode>>) -> bool {
    node.borrow().left == None && node.borrow().right == None
}