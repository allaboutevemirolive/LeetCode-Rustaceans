// https://leetcode.com/problems/rotate-list/solutions/2428708/rust-solution/
use std::collections::*;
impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if let Some(head) = head {
            if head.next.is_none() {
                return Some(head)
            }

            let mut que = VecDeque::new();
            let mut node = Some(head);
            while let Some(mut inner) = node {
                let next = inner.next;
                inner.next = None;
                que.push_back(inner);
                node = next;
            } 

            let k = k as usize % que.len();
            for _ in 0..k {
                let node = que.pop_back().unwrap();
                que.push_front(node);
            }
            while que.len() > 1 {
                let node = que.pop_back().unwrap();
                let li = que.len() - 1;
                que[li].next = Some(node);
            }
            
            Some(que.pop_back().unwrap())
        }  else {
            None
        }  
    }
}