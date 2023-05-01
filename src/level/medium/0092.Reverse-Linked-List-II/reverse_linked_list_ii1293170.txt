// https://leetcode.com/problems/reverse-linked-list-ii/solutions/1293170/rust-a-poor-man-s-vec-n-recreate-solution/
impl Solution {
    pub fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
        if left == right { return head }
        let mut stack = vec![];
        let mut next = head;
        while let Some(this) = next {
            stack.push(this.val);
            next = this.next;
        }
        stack[left as usize - 1..=right as usize - 1].reverse();
        let mut next = None;
        while let Some(val) = stack.pop() {
            next = Some(Box::new(ListNode { next, val }));
        }
        next
    }
}