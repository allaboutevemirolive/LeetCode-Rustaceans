// https://leetcode.com/problems/partition-list/solutions/2319214/rust-2ms-easy-two-lists/
use std::cmp;

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
impl Solution {
    pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        if let Some(node) = &head {
            if node.next == None {
                return head;
            }
        } else {
            return None;
        }

        let mut smaller: Option<Box<ListNode>> = None;
        let mut smaller_tail: &mut Option<Box<ListNode>> = &mut None;
        let mut greater: Option<Box<ListNode>> = None;
        let mut greater_tail: &mut Option<Box<ListNode>> = &mut None;

        while let Some(mut node) = head {
            head = node.next.take();
            match node.val.cmp(&x) {
                cmp::Ordering::Less => {
                    if let Some(stnode) = smaller_tail.as_mut() {
                        stnode.next = Some(node);
                        smaller_tail = &mut stnode.next;
                    } else {
                        smaller = Some(node);
                        smaller_tail = &mut smaller;
                    }
                }
                cmp::Ordering::Equal | cmp::Ordering::Greater => {
                    if let Some(gtnode) = greater_tail.as_mut() {
                        gtnode.next = Some(node);
                        greater_tail = &mut gtnode.next;
                    } else {
                        greater = Some(node);
                        greater_tail = &mut greater;
                    }
                }
            }
        }

        if let Some(stnode) = smaller_tail.as_mut() {
            stnode.next = greater;
            smaller
        } else {
            greater
        }
        
    }
}