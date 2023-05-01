// https://leetcode.com/problems/add-two-numbers/solutions/3327841/rust-solution/
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
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // fn reverse(mut l: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        //     let mut prev = None;
        //     while let Some(mut cur) = l.take() {
        //         let next = cur.next.take();
        //         cur.next = prev;
        //         prev = Some(cur);
        //         l = next;
        //     }
        //     prev
        // }

        let mut result_head: Option<Box<ListNode>> = None;
        let mut result_tail = &mut result_head;

        // let mut l1 = reverse(l1);
        let mut l1_tail = l1.as_mut();
        // let mut l2 = reverse(l2);
        let mut l2_tail = l2.as_mut();
        let mut carry = 0;
        loop {
            let num1 = if let Some(n) = l1_tail {
                l1_tail = n.next.as_mut();
                n.val
            } else {
                0
            };
            let num2 = if let Some(n) = l2_tail {
                l2_tail = n.next.as_mut();
                n.val
            } else {
                0
            };

            let sum = num1 + num2 + carry;
            let x = result_tail.insert(Box::new(ListNode {
                val: sum % 10,
                next: None,
            }));
            result_tail = &mut x.next;
            carry = sum / 10;

            if l1_tail.is_none() && l2_tail.is_none() {
                if carry > 0 {
                    let _ = result_tail.insert(Box::new(ListNode {
                        val: carry,
                        next: None,
                    }));
                }
                break;
            }
        }

        result_head        
    }
}