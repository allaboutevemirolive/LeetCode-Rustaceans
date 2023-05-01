// https://leetcode.com/problems/partition-list/solutions/3331416/rust-no-vec/
impl Solution {
    pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut dummy1 = ListNode::new(-1);
        let mut dummy2 = ListNode::new(-1);
        let mut r1 = &mut dummy1;
        let mut r2 = &mut dummy2;
        while let Some(mut node) = head.take() {
            head = node.next.take();
            if node.val < x {
                r1.next = Some(node);
                r1 = r1.next.as_mut().unwrap();
            } else {
                r2.next = Some(node);
                r2 = r2.next.as_mut().unwrap();
            }
        }
        r1.next = dummy2.next;
        dummy1.next
    }
}