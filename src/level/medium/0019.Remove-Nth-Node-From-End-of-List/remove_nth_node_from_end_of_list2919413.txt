// https://leetcode.com/problems/remove-nth-node-from-end-of-list/solutions/2919413/rust-2-pointer/
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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut sentinel = Some(Box::new(ListNode {
            val: -1,
            next: head
        }));
        let mut r = &(sentinel.clone());
        for _ in 0..n {
            r = &r.as_ref()?.next;
        }
        let mut l = &mut sentinel;
        while r.is_some() && r.as_ref()?.next.is_some() {
            l = &mut l.as_mut()?.next;
            r = &r.as_ref()?.next;
        }
        l.as_mut()?.next = l.as_mut()?.next.as_mut()?.next.take();
        sentinel?.next
    }
}