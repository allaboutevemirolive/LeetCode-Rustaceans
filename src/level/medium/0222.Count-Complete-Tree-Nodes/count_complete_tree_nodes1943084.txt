// https://leetcode.com/problems/count-complete-tree-nodes/solutions/1943084/rust-count-height-of-left-and-right/
impl Solution {
    fn count_left(mut root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(root) => {
                1 + Self::count_left(root.borrow().left.clone())
            }
        }
    }

    fn count_right(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(root) => {
                1 + Self::count_right(root.borrow().right.clone())
            }
        }
    }

    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(root) => {
                let left = Self::count_left(root.borrow().left.clone());
                let right = Self::count_right(root.borrow().right.clone());
                
                if left == right {
                    return (1 << (left + 1)) - 1;
                }

                1 + Self::count_nodes(root.borrow().left.clone())
                    + Self::count_nodes(root.borrow().right.clone())
            }
        }
    }
}