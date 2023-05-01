// https://leetcode.com/problems/binary-search-tree-iterator/solutions/1072171/rust-stack-solution-with-unit-test/
use std::cell::RefCell;
use std::rc::Rc;
pub type OrrTreeNode = Option<Rc<RefCell<TreeNode>>>;

struct BSTIterator {
    stack: Vec<OrrTreeNode>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl BSTIterator {
    fn new(root: OrrTreeNode) -> Self {
        let stack: Vec<OrrTreeNode> = vec![];
        let mut this = BSTIterator { stack: stack };
        this.expand_left(root);
        this
    }

    fn next(&mut self) -> i32 {
        if let Some(node) = self.stack.pop().unwrap() {
            let val = node.borrow().val;
            self.expand_left(node.borrow().right.clone());
            return val;
        }
        -1
    }

    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }

    fn expand_left(&mut self, root: OrrTreeNode) {
        let mut node = root;
        while node.is_some() {
            self.stack.push(node.clone());
            node = node.unwrap().borrow().left.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ptsutil::codec::Codec;

    #[test]
    fn test_solution() {
        let root = Codec::deserialize("7,3,15,#,#,9,20,#,#,#,#".to_string());
        let mut iter = BSTIterator::new(root);
        assert_eq!(3, iter.next()); // return 3
        assert_eq!(7, iter.next()); // return 7
        assert_eq!(true, iter.has_next()); // return True
        assert_eq!(9, iter.next()); // return 9
        assert_eq!(true, iter.has_next()); // return True
        assert_eq!(15, iter.next()); // return 15
        assert_eq!(true, iter.has_next()); // return True
        assert_eq!(20, iter.next()); // return 20
        assert_eq!(false, iter.has_next()); // return False
    }
}