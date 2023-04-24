// https://leetcode.com/problems/recover-a-tree-from-preorder-traversal/solutions/547873/rust-clean-code-0ms/
use std::cell::RefCell;
use std::collections::LinkedList;
use std::rc::Rc;

struct NumberAndDepth<'a>(usize, usize, &'a str);

impl Solution {
    pub fn recover_from_preorder(s: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut rest: &str = &s;
        let mut stack = LinkedList::new();

        while rest.len() != 0 {
            let NumberAndDepth(val, dashes_count, rest_str) = read_dashes_and_number(rest);
            rest = rest_str;

            truncate(&mut stack, dashes_count);
            insert_new_node(&mut stack, val as i32);
        }

        let root = stack.front().expect("Tree root");
        Some(Rc::clone(root))
    }
}

fn insert_new_node(stack: &mut LinkedList<Rc<RefCell<TreeNode>>>, val: i32) {
    let new_node = Rc::new(RefCell::new(TreeNode::new(val)));

    if stack.len() == 0 {
        stack.push_back(new_node);
        return;
    }

    let current_node = Rc::clone(stack.back().expect("Previous node"));
    stack.push_back(Rc::clone(&new_node));

    if current_node.borrow().left == None {
        current_node.borrow_mut().left = Some(new_node);
    } else {
        current_node.borrow_mut().right = Some(new_node);
    }
}

fn truncate(stack: &mut LinkedList<Rc<RefCell<TreeNode>>>, size: usize) {
    while stack.len() > size {
        stack.pop_back();
    }
}

fn read_dashes_and_number(s: &str) -> NumberAndDepth {
    let (dashes_count, rest) = read_dashes(s);
    let (number, rest) = read_number(rest);

    NumberAndDepth(number, dashes_count, rest)
}

fn read_number(s: &str) -> (usize, &str) {
    let number_str: String = s.chars().take_while(|c| c.is_digit(10)).collect();
    let number: usize = number_str.parse().expect("Number");

    (number, &s[number_str.len()..])
}

fn read_dashes(s: &str) -> (usize, &str) {
    let dashes_count = s.chars().take_while(|&c| c == '-').count();

    (dashes_count, &s[dashes_count..])
}