// https://leetcode.com/problems/partition-list/solutions/2787977/rust-solution-no-copy/
use std::mem;
impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let (mut lt_head, mut gte_head) = (None, None);
        let (mut lt_tail, mut gte_tail) = (&mut lt_head, &mut gte_head);
        while let Some(mut node) = head {
            head = node.next.take();
            if node.val < x {
                lt_tail = &mut lt_tail.insert(node).next;
            } else {
                gte_tail = &mut gte_tail.insert(node).next;
            }
        }
        mem::replace(lt_tail, gte_head);
        lt_head
    }
}