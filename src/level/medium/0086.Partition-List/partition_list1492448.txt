// https://leetcode.com/problems/partition-list/solutions/1492448/rust-0ms-clean/
pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    let (mut lhead, mut hhead) = (None, None);
    let (mut low, mut high) = (&mut lhead, &mut hhead);
    while let Some(mut node) = head {
        head = node.next.take();
        if node.val < x {
            *low = Some(node);
            low = &mut low.as_deref_mut().unwrap().next;
        } else {
            *high = Some(node);
            high = &mut high.as_deref_mut().unwrap().next;
        }
    }
    *low = hhead;
    lhead
}