// https://leetcode.com/problems/binary-search-tree-iterator/solutions/868442/rust-8ms-stack-solution/
use std::rc::Rc;
use std::cell::RefCell;
struct BSTIterator {
    current: Option<Rc<RefCell<TreeNode>>>,
    path: Vec<Rc<RefCell<TreeNode>>>,
}

impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        match root {
            Some(mut node) => {
                let mut path = vec![];
                while let Some(left) = node.clone().borrow().left.clone() {
                    path.push(node);
                    node = left;
                }
                Self {
                    current: Some(node),
                    path,
                }
            },
            None => Self {
                current: None,
                path: vec![],
            },
        }
    }
    
    /** @return the next smallest number */
    fn next(&mut self) -> i32 {
        let result = self.current.as_ref().unwrap().borrow().val;
        let right_option = self.current.as_ref().unwrap().borrow().right.clone();
        self.current = if let Some(mut node) = right_option {
            while let Some(left) = node.clone().borrow().left.clone() {
                self.path.push(node);
                node = left;
            }
            Some(node)
        } else {
            self.path.pop()
        };
        result
    }
    
    /** @return whether we have a next smallest number */
    fn has_next(&self) -> bool {
        self.current.is_some()
    }
}