// https://leetcode.com/problems/rotate-list/solutions/2831149/rust-0ms-no-extra-space/
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
type Node = Option<Box<ListNode>>;

impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        fn rot(mut head: Node) -> Node {
            let mut tail = &mut head;
            while tail.is_some() && tail.as_ref()?.next.is_some() {
                tail = &mut tail.as_mut()?.next;
            }
            let mut tail = tail.take();
            tail.as_mut()?.next = head;
            tail
        }

        fn size(head: &Node) -> i32 {
            let mut size = 0;
            let mut cur = head;
            while let Some(node) = cur {
                cur = &node.as_ref().next;
                size += 1;
            }
            size
        }
        
        if head.is_none() {
            return head;
        }

        let size = size(&head);
        let mut res = head;
        for _ in 0..k % size {
            res = rot(res)
        }
        res
    }
}