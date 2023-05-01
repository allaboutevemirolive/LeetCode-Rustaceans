// https://leetcode.com/problems/sort-list/solutions/3330099/rust-solution/
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
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn get_mid(head: Option<&mut Box<ListNode>>) -> Option<Box<ListNode>> {
            fn get_len(head: Option<&Box<ListNode>>) -> i32 {
                let mut len = 0;
                let mut cur = head;
                while let Some(c) = cur {
                    cur = c.next.as_ref();
                    len += 1;
                }
                len
            }
            let len = get_len(head.as_deref());
            let mut pre_mid_idx = len / 2 - 1;
            let mut cur = head;
            while pre_mid_idx > 0 {
                cur = cur.unwrap().next.as_mut();
                pre_mid_idx -= 1;
            }
            cur.unwrap().next.take()
        }

        fn merge_list(
            mut left: Option<Box<ListNode>>,
            mut right: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            if let Some(mut left) = left.take() {
                if let Some(mut right) = right.take() {
                    if left.val <= right.val {
                        left.next = merge_list(left.next.take(), Some(right));
                        Some(left)
                    } else {
                        right.next = merge_list(Some(left), right.next.take());
                        Some(right)
                    }
                } else {
                    Some(left)
                }
            } else {
                right
            }
        }

        fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            if head.is_none() || head.as_ref().unwrap().next.is_none() {
                return head;
            }
            let mid = get_mid(head.as_mut());
            
            // println!("h {:?} m {:?}", head, mid);

            let left = sort_list(head);
            let right = sort_list(mid);

            // println!("l {:?} r {:?}", left, right);

            merge_list(left, right)
        }

        sort_list(head)        
    }
}