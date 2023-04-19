// https://leetcode.com/problems/count-of-smaller-numbers-after-self/solutions/531481/rust-bst/
use std::rc::Rc;
use std::cell::RefCell;

struct Node {
    val: i32,
    count: usize,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>
}

impl Solution {
    
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; nums.len()];
        let mut root = None;
        
        for k in 0..nums.len() {
            let i = nums.len() - 1 - k;
            
            let node = Node {
                val: nums[i],
                count: 0,
                left: None,
                right: None
            };
            
            root = Self::insert(root, Rc::new(RefCell::new(node)), &mut res, i);
        }
        
        res
    }
    
    pub fn insert (
        root: Option<Rc<RefCell<Node>>>, 
        node: Rc<RefCell<Node>>, 
        mut res: &mut Vec<i32>,
        i: usize
    ) -> Option<Rc<RefCell<Node>>> {
        if let None = root { return Some(node.clone()); }
        
        let rt = root.unwrap();
        let val = rt.borrow().val;
        
        if val >= node.borrow().val {
            rt.borrow_mut().count += 1;
            
            let left = Self::insert(rt.borrow().left.clone(), node.clone(), &mut res, i);
            rt.borrow_mut().left = left;
        } else {
            res[i] += (rt.borrow().count + 1) as i32;
            
            let right = Self::insert(rt.borrow().right.clone(), node.clone(), &mut res, i);
            rt.borrow_mut().right = right;
        }
        
        return Some(rt.clone());
    } 
}