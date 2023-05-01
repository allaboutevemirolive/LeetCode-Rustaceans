// https://leetcode.com/problems/sort-list/solutions/560863/merge-sort-in-rust/
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
        let (lhs,rhs) = Self::split(head);
        match (lhs,rhs) {
            (None,None) => None,
            (lhs,None) => lhs,
            (None,rhs) => rhs,
            (lhs,rhs) => Self::merge(Self::sort_list(lhs),Self::sort_list(rhs)),
        }
    }
    
    fn split(mut head : Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        let mut lhs = None;
        let mut rhs = None;
        let mut i = true;
        loop{
            head = match head {
                None => break,
                Some(mut headnode) => {
                    let head = headnode.next.take();
                    match i {
                        true => {
                            headnode.next = lhs.take();
                            lhs = Some(headnode);
                        },
                        false => {
                            headnode.next = rhs.take();
                            rhs = Some(headnode);
                        },
                    }
                    head
                },
            };
            i = !i;
        };
        (lhs,rhs)
    }
    
    fn merge(lhs : Option<Box<ListNode>>, rhs : Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (lhs,rhs) {
            (None,None) => None,
            (node,None) => node,
            (None,node) => node,
            (Some(mut h), Some(mut t)) => {
                let (mut small,large) = if h.val < t.val {
                                (h,t)
                            } else {
                                (t,h)    
                            };
                let successor = small.next.take();
                small.next=Self::merge(successor,Some(large));
                Some(small)
            }
        }
    }
}