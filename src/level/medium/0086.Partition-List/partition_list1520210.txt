// https://leetcode.com/problems/partition-list/solutions/1520210/vec-solution-rust-2mb/
impl Solution {
    pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut left: Vec<i32> = vec![];
        let mut right: Vec<i32> = vec![];
        while let Some(node) = head {
            let val = node.val;
            if val < x {
                left.push(val);
            } else {
                right.push(val);
            }
            head = node.next;
        }
        let mut next = None;
        while let Some(val) = right.pop() {
            next = Some(Box::new(ListNode{next, val}));
        }
        while let Some(val) = left.pop() {
            next = Some(Box::new(ListNode{next, val}));

        }
        next
    }
}