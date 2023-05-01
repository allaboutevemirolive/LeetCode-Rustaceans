// https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/solutions/2836793/rust-simple-loop-with-sentinel/
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut sentinel = ListNode { val: 0, next: head };

        let mut pred = &mut sentinel;

        while let Some(mut h) = pred.next.as_mut() {
            if h.next.is_some() && h.val == h.next.as_ref().unwrap().val {
                while h.next.is_some() && h.val == h.next.as_ref().unwrap().val {
                    h = h.next.as_mut().unwrap();
                }
                pred.next = h.next.take();
            } else {
                pred = pred.next.as_mut().unwrap();
            }
        }

        sentinel.next
    }