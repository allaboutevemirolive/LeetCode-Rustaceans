// https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/solutions/931496/rust-cheapest-best/
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut to_remove = std::i32::MIN;
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut ptr = dummy.as_mut();
        while let Some(node) = ptr {
            match (
                node.next.as_ref().and_then(|n| Some(n.val)),
                node.next
                    .as_ref()
                    .and_then(|n| n.next.as_ref().and_then(|m| Some(m.val))),
            ) {
                (Some(a), _) if a == to_remove => {
                    node.next = node.next.as_mut().unwrap().next.take();
                    ptr = Some(node);
                }
                (Some(a), Some(b)) if a == b => {
                    to_remove = a;
                    ptr = Some(node);
                }
                _ => {
                    ptr = node.next.as_mut();
                }
            }
        }

        dummy.unwrap().next.take()
    }
}