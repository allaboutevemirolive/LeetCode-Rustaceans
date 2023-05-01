// https://leetcode.com/problems/binary-search-tree-iterator/solutions/753637/rust-cheapest-best/
use std::rc::Rc;
use std::cell::RefCell;

struct BSTIterator {
    iter: std::iter::Peekable<Box<dyn Iterator<Item = i32>>>,
}

impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        Self {
            iter: Self::go(&root).peekable(),
        }
    }

    fn next(&mut self) -> i32 {
        self.iter.next().unwrap()
    }

    fn has_next(&mut self) -> bool {
        self.iter.peek().is_some()
    }

    fn go(node: &Option<Rc<RefCell<TreeNode>>>) -> Box<dyn Iterator<Item = i32>> {
        match node {
            None => Box::new(0..0),
            Some(n) => {
                let v = n.borrow().val;
                Box::new(
                    Self::go(&n.borrow().left)
                        .chain(v..=v)
                        .chain(Self::go(&n.borrow().right)),
                )
            }
        }
    }
}