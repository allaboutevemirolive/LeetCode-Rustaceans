// https://leetcode.com/problems/rotate-list/solutions/931460/rust-cheapest-best/
use std::collections::VecDeque;

impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, mut k: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }

        let mut len = 1;
        let mut q: VecDeque<Box<ListNode>> = VecDeque::new();
        let mut head = head.unwrap();
        while let Some(node) = head.next.take() {
            len += 1;
            q.push_back(head);
            head = node;
        }
        q.push_back(head);

        if k >= len {
            k = k % len;
        }

        for _ in 0..k {
            let next = q.pop_back().unwrap();
            q.push_front(next);
        }

        head = q.pop_front().unwrap();
        let mut ptr = &mut head;
        while let Some(node) = q.pop_front() {
            ptr.next = Some(node);
            ptr = ptr.next.as_mut().unwrap();
        }

        Some(head)
    }
}