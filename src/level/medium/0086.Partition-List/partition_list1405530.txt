// https://leetcode.com/problems/partition-list/solutions/1405530/simple-rust-solution-o-n-0ms/
impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut lt = Some(Box::new(ListNode::new(-1000)));
        let mut ge = Some(Box::new(ListNode::new(-1000)));
        let mut lt_curr = lt.as_mut();
        let mut ge_curr = ge.as_mut();
        let mut curr = head;

        while let Some(n) = curr {
            if n.val < x {
                if let Some(nn) = lt_curr {
                    nn.next = Some(n.clone());
                    lt_curr = nn.next.as_mut();
                }
            } else {
                if let Some(nn) = ge_curr {
                    nn.next = Some(n.clone());
                    ge_curr = nn.next.as_mut();
                }
            }

            curr = n.next;
        }

        ge_curr.unwrap().next = None;
        lt_curr.unwrap().next = ge.unwrap().next;
        lt.unwrap().next
    }
}