// https://leetcode.com/problems/partition-list/solutions/826370/rust-safe-time-complexity-o-n-space-complexity-o-1-0ms-2mb/
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
        if head.is_none() {
            return head;
        }

        // before and after are the two pointers used to create the two list
        // before_head and after_head are used to save the heads of the two lists.
        // All of these are initialized with the dummy nodes created.
        let mut before_head = Box::new(ListNode {
            val: -1,
            next: None,
        });
        let mut after_head = Box::new(ListNode::new(-1));

        let mut before = &mut before_head;
        let mut after = &mut after_head;
        while let Some(mut node) = head {
            // If the original list node is lesser than the given x,
            // assign it to the before list.
            if node.val < x {
                // move ahead in the original list
                head = node.next.take();
                before.next = Some(node);
                before = before.next.as_mut().unwrap();
            } else {
                // move ahead in the original list
                head = node.next.take();
                // If the original list node is greater or equal to the given x,
                // assign it to the after list.
                after.next = Some(node);
                after = after.next.as_mut().unwrap();
            }
        }
        after.next = None;

        // Once all the nodes are correctly assigned to the two lists,
        // combine them to form a single list which would be returned.
        before.next = after_head.next;
        before_head.next
    }

}