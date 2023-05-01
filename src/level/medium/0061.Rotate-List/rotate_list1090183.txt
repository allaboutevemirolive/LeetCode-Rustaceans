// https://leetcode.com/problems/rotate-list/solutions/1090183/idiomatic-rust-solution-32ms-2mb/
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

impl ListNode {
    
    fn push(&mut self, val: i32) {
        
        let new_node = ListNode {
            val,
            next: Some(Box::new(self.clone()))
        };
        
        // Dereference self, and assign it the new node.
        *self = new_node
    }
    
    fn pop_tail(&mut self) -> i32 {
        
        // 2 pointer technique.
        let mut dummy = ListNode{
            val: -1,
            next: Some(Box::new(self.clone())) // This is the head of the dummy.
        };
        
        let mut right = self.clone();
        let mut left = &mut dummy; // Mutable reference... one behind right. We will use this to update the dummy list without moving it's head.
        
        while let Some(n) = right.next {
            // move left forward 1, making sure it remains a mutable reference.
            left = left.next.as_mut().unwrap();
            // move right forward 1.
            right = *n
        }
        
        // Right has moved to the end.
        // Left is one behind it.
        left.next = None;
        
        // Assign self to the head of dummy.
        *self = *dummy.next.unwrap();
        
        right.val
        
    }
    

    fn length(&mut self) -> i32 {
        let mut i = 1;
        // Need a dummy to destructively work through.
        let mut dummy = self.clone();
        while let Some(mut n) = dummy.next {
            dummy = *n;
            i+=1
        };
        
        i
        
    }
    
}


impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        match head {
            Some(mut n) => {

                let mut i = 0;
                let mut len = &n.length();
                let mut times = len + k.rem_euclid(*len);
                
                // Edge Cases.
                if (k == 0) {
                    Some(n)
                } else if (*len == 1) {
                    Some(n)
                } else if (*len == times) {
                    Some(n)
                } else {
                    while i < times {
                        // Pop the tail.
                        let mut tail = n.pop_tail();
                        // Push the former tail.
                        n.push(tail);    
                        i += 1;
                    }
                    Some(n)
                }
            },
            None => {
                head
            }
        }
    }
}