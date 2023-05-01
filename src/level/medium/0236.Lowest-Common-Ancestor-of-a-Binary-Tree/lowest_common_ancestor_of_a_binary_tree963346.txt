// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/solutions/963346/rust-recursive/
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let root = match root {
            Some(a) => a,
            None => return None,
        };
        let v1 = p.unwrap().borrow().val;
        let v2 = q.unwrap().borrow().val;
        Self::helper(root, v1, v2)
    }

    fn helper(root: Rc<RefCell<TreeNode>>, v1: i32, v2: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let root_v = root.borrow().val;
        if root_v == v1 || root_v == v2 {
            return Some(root);
        }
        let l = root
            .borrow()
            .left
            .as_ref()
            .and_then(|a| Self::helper(a.clone(), v1, v2));
        let r = root
            .borrow()
            .right
            .as_ref()
            .and_then(|a| Self::helper(a.clone(), v1, v2));
        match (l, r) {
            (Some(_), Some(_)) => Some(root),
            (None, None) => None,
            (Some(l), None) => Some(l),
            (None, Some(r)) => Some(r),
        }
    }
}