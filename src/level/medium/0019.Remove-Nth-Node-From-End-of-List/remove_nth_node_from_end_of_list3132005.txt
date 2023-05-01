// https://leetcode.com/problems/remove-nth-node-from-end-of-list/solutions/3132005/optimus-rust/
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
        let mut head = head;
        let mut len = 0;
        let mut curr = &head;
        while let Some(node) = curr {
            len += 1;
            curr = &node.next;
        }

        let mut curr = &mut head;
        let mut i = 0;

        if len == n {
            return head.and_then(|x| x.next);
        }

        while let Some(node) = curr {
            i += 1;
            if i == len - n {
                let next = node.next.take();
                node.next = next.and_then(|x| x.next);
                break;
            }
            curr = &mut node.next;
        }

        head

    }
}
