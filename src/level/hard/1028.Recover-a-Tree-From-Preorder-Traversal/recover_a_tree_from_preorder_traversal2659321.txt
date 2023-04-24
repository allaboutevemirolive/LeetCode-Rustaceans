// https://leetcode.com/problems/recover-a-tree-from-preorder-traversal/solutions/2659321/rust-iterative-solution-with-comments/
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn recover_from_preorder(traversal: impl AsRef<str>) -> Option<Rc<RefCell<TreeNode>>> {
    let traversal = traversal.as_ref();

    // The first element of the tuple is the expected depth of this node's children
    // The second element is the node itself
    let mut stack: Vec<(i32, Rc<RefCell<TreeNode>>)> = vec![];

    // The root starts at depth=0
    let mut depth = 0;

    // Parse the input string, by splitting it on the `-` character
    // The `split()` function removes the `-` character from the result,
    // thus a string like `1-`, results in only `[1]`, and strings like
    // `1--` result in `[1, ""]`, thus we have to account for the "lost"
    // `-` character in order to compute the correct "depth"
    for node_val in traversal.split('-') {
        // an empty string means that both the current and the previous
        // characters were `-`, thus we have to increment the depth
        // counter of the node that is to come
        if node_val.is_empty() {
            depth += 1;
            continue;
        }

        let value = node_val.parse().unwrap();
        let child = Rc::new(RefCell::new(TreeNode::new(value)));

        if !stack.is_empty() {
            // Because the raw string is in "preorder traversal" order,
            // then if the current depth is equal to the expected depth,
            // then this is the left child of the last pushed node.
            if stack[stack.len() - 1].0 == depth {
                stack[stack.len() - 1].1.borrow_mut().left = Some(child.clone());
            } else {
                // Otherwise, this is the right child of some previously pushed node,
                // for which we have already set the left child. Thus we have to
                // continuously pop nodes from the stack, until we reach a node with
                // the expected depth.
                //
                // It will never cause an "out of bounds" error, because no node can
                // have an expected depth of less than 1, which is the expected depth
                // of the root's children
                while stack[stack.len() - 1].0 > depth {
                    stack.pop();
                }
                stack[stack.len() - 1].1.borrow_mut().right = Some(child.clone());
            }
        }

        // push the expected depth of the children of this node
        stack.push((depth + 1, child));

        // reset the depth counter to 1, because `.split()` removes the
        // first separator, but we still need to count it
        depth = 1;
    }

    Some(stack[0].1.clone())
}