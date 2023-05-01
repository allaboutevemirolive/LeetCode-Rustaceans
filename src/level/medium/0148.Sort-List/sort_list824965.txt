// https://leetcode.com/problems/sort-list/solutions/824965/rust-safe-merge-sort-time-complexity-o-nlogn-space-complexity-o-1-8ms-4-2mb/
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
    #[inline]
    pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let len = Self::length(&head);

        if len < 2 {
          return head;
        }
        let mut ptr = head.as_mut();
        for _ in 0..(len / 2)  - 1{
            if let Some(node) = ptr {
                ptr = node.next.as_mut();
            }
        }

        let (mut right, mut left) = (ptr.unwrap().next.take(), head);

        Self::merge_two_lists(Self::sort_list(left), Self::sort_list(right))

    }
    fn length(mut head: &Option<Box<ListNode>>) -> usize {
        let mut count = 0;
        while let Some(node) = head {
            head = &node.next;
            count += 1;
        }
        count
    }
        fn merge_two_lists(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result = ListNode::new(0);
        let mut p = &mut result;

        while l1.is_some() && l2.is_some() {
            let deref_val = |node: &Box<ListNode>| (*node).val;
            let digit1 = l1.as_ref().map_or(0, deref_val);
            let digit2 = l2.as_ref().map_or(0, deref_val);
            if digit1 <= digit2 {
                let tmp = l1.as_mut().unwrap().next.take();
                p.next = l1.take();
                l1 = tmp;
            } else {
                let tmp = l2.as_mut().unwrap().next.take();
                p.next = l2.take();
                l2 = tmp;
            }
            p = p.next.as_mut().unwrap();
        }
      if l1.is_none() {
          p.next = l2.take();
        }else {
          p.next = l1.take();
        }
        result.next
    }

}