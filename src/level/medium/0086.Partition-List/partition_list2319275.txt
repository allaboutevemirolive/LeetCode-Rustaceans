// https://leetcode.com/problems/partition-list/solutions/2319275/rust-solution-no-vector/
impl Solution {
    pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut lo_head = ListNode::new(0);
        let mut hi_head = ListNode::new(0);

        let mut lo_tail= &mut lo_head;
        let mut hi_tail= &mut hi_head;

        while let Some(mut cur) = head.take() {
            head = cur.next.take();

            if cur.val < x {
                lo_tail.next = Some(cur);
                lo_tail = lo_tail.next.as_mut().unwrap();
            } else {
                hi_tail.next = Some(cur);
                hi_tail = hi_tail.next.as_mut().unwrap();
            }
        }

        lo_tail.next = hi_head.next.take();


        lo_head.next.take()
    }
}