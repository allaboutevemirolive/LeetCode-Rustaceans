// https://leetcode.com/problems/rotate-list/solutions/3352130/rust-solution/
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
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() || k == 0 {
            return head;
        }
        fn reverse(mut head: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, i32) {
            let mut len = 0;
            let mut prev = None;
            while let Some(mut h) = head {
                let next = h.next.take();
                h.next = prev;
                prev = Some(h);
                head = next;
                len += 1;
            }
            (prev, len)
        }
        fn split(
            mut head: Option<Box<ListNode>>,
            mut at: i32,
        ) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
            let mut pre_tail = &mut head;
            while at > 1 {
                if let Some(pt) = pre_tail {
                    pre_tail = &mut pt.next;
                }
                at -= 1;
            }
            let tail = pre_tail.as_mut().unwrap().next.take();
            (head, tail)
        }

        let (head, len) = reverse(head);

        let at = k % len;
        if at == 0 {
            return reverse(head).0;
        }

        let (head, tail) = split(head, at);
        let (mut head, _) = reverse(head);
        let (tail, _) = reverse(tail);

        let mut head_tail = &mut head;
        while let Some(ht) = head_tail {
            head_tail = &mut ht.next;
        }
        if let Some(tail) = tail {
            let _ = head_tail.insert(tail);
        }
        head
    }
}