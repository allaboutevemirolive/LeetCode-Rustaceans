// https://leetcode.com/problems/rotate-list/solutions/1839276/rust-recursive/
impl Solution {
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        let len = Self::length(&head);
        let k = k % len;
        if k == 0 {
            return head;
        }
        let mut new_head = Self::nth_node(&mut head, len - k);
        Self::append_end(&mut new_head, head);
        new_head
    }
    
    fn length(node: &Option<Box<ListNode>>) -> i32 {
        match node {
            None => 0,
            Some(n) => 1 + Self::length(&n.next),
        }
    }
    
    fn nth_node(node: &mut Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        if n == 1 {
            return node.as_mut().unwrap().next.take();
        }
        Self::nth_node(&mut node.as_mut().unwrap().next, n - 1)
    }
    
    fn append_end(mut node: &mut Option<Box<ListNode>>, add: Option<Box<ListNode>>) {
        let n = node.as_mut().unwrap();
        match n.next {
            None => n.next = add,
            Some(_) => Self::append_end(&mut n.next, add),
        };
    }
}