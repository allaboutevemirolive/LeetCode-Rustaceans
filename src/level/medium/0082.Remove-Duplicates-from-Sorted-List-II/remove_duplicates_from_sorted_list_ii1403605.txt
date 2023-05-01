// https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/solutions/1403605/recursive-rust-short-and-easy-to-understand-0ms-beats-100/
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn go(node: Option<Box<ListNode>>, prev: Option<i32>) -> Option<Box<ListNode>> {
            if let Some(mut n) = node {
                let x = Some(n.val);
                if x == prev {
                    go(n.next, prev)
                } else {
                    match n.next {
                        Some(n2) if n2.val == n.val => go(Some(n2), x),
                        _ => {
                            n.next = go(n.next, x);
                            Some(Box::new(*n))
                        }
                    }
                }
            } else {
                None
            }
        }

        go(head, None)
    }
}