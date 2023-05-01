// https://leetcode.com/problems/binary-search-tree-iterator/solutions/1966026/rust-iterative-move-semantics-no-clone/
// Type aliases to reduce verbosity
type NodeRc = Rc<RefCell<TreeNode>>;
type NodeRcOption = Option<NodeRc>;

use std::{cell::RefCell, mem, rc::Rc, vec};
struct BSTIterator {
    // Stack to keep current branch
    stack: Vec<NodeRc>,
}

impl BSTIterator {
    fn new(root: NodeRcOption) -> Self {
        // Initialize stack by following the leftmost branch
        Self {
            stack: Self::walk_left(vec![], root),
        }
    }

    fn walk_left(mut stack: Vec<NodeRc>, mut root: NodeRcOption) -> Vec<NodeRc> {
        // Walk left until reaching the leaf
        while let Some(node) = root {
            let left = node.borrow_mut().left.take();
            stack.push(node);
            root = left;
        }
        stack
    }

    fn next(&mut self) -> i32 {
        let node = self.stack.pop().unwrap();
        let mut node = node.borrow_mut();
        // Walk down branch to the right, if any from this node
        self.stack = Self::walk_left(mem::take(&mut self.stack), node.right.take());
        node.val
    }

    fn has_next(&self) -> bool {
        // If the stack is empty and there is no current element,
        // there is no more work to do
        !self.stack.is_empty()
    }
}