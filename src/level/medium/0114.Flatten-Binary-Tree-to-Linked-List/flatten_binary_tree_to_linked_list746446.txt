// https://leetcode.com/problems/flatten-binary-tree-to-linked-list/solutions/746446/rust-beats-100/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut vals: Vec<i32> = Vec::new();
        Self::read(root, &mut vals);
        Self::build(root, vals);
    }
    
    pub fn build(mut root: &mut Option<Rc<RefCell<TreeNode>>>, vals: Vec<i32>)
    {
        if vals.is_empty()
        {
            return;
        }
        
        if root.is_none()
        {
            *root = Some(Rc::new(RefCell::new(TreeNode::new(vals[0]))));
        }
        else
        {
            root.as_mut().unwrap().borrow_mut().val = vals[0];
        }
        root.as_mut().unwrap().borrow_mut().left = None;
        Self::build(&mut root.as_mut().unwrap().borrow_mut().right, vals[1..].to_vec());
    }
    
    pub fn read(root: &mut Option<Rc<RefCell<TreeNode>>>, vals: &mut Vec<i32>)
    {
        if root.is_none()
        {
            return;
        }
        
        vals.push(root.as_mut().unwrap().borrow().val);
        Self::read(&mut root.as_mut().unwrap().borrow_mut().left, vals);
        Self::read(&mut root.as_mut().unwrap().borrow_mut().right, vals);
    }
}