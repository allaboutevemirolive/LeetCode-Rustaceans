// https://leetcode.com/problems/validate-binary-search-tree/solutions/2412664/rust-solution/
impl Solution {
    fn valid_bst(node: Option<&Rc<RefCell<TreeNode>>>) -> (bool, i32, i32) {
        let N = node.unwrap();
        let val = N.borrow().val;
        
        match (N.borrow().left.as_ref(), N.borrow().right.as_ref()) {
            (Some(left), Some(right)) => {
                let l = Solution::valid_bst(Some(left));
                let r = Solution::valid_bst(Some(right));
                // subtrees valid and left max < root val < right min
                (l.0 && l.2 < val && r.0 && r.1 > val, 
                    val.min(l.1.min(r.1)), val.max(l.2.max(r.2)))
            },
            (Some(left), None) => {
                let l = Solution::valid_bst(Some(left));
                // left subtree valid and left max < root val
                (l.0 && l.2 < val, val.min(l.1), val.max(l.2))
            },
            (None, Some(right)) => {
                let r = Solution::valid_bst(Some(right));
                // right subtree valid and right min > root val
                (r.0 && r.1 > val, val.min(r.1), val.max(r.2))
            },
            (None, None) => {
                // (valid, min, max)
                (true, val, val)
            }    
        }
    }
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::valid_bst(root.as_ref()).0
    }
}