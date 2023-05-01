// https://leetcode.com/problems/reverse-linked-list-ii/solutions/1195254/rust-clean-solution-one-pass-time-o-n-space-o-1/
pub fn reverse_between(
    head: Option<Box<ListNode>>,
    left: i32,
    right: i32,
) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
    let mut prev = &mut dummy;
    for _ in 1..left { // notice that `left` and `right` start at `1`
        prev = &mut prev.as_mut().unwrap().next;
    }
    let mut curr = prev.as_mut().unwrap().next.take(); // the start point of reversing
    for _ in left..right {
        let mut node = curr.as_mut().unwrap().next.take(); // take the next chain of current node
        curr.as_mut().unwrap().next = node.as_mut().unwrap().next.take(); // reserve the next chain of `node` to current node, now `node` is a 'single' node
        node.as_mut().unwrap().next = prev.as_mut().unwrap().next.take(); // insert `node` ahead of current node, that why call it 'head insertion'
        prev.as_mut().unwrap().next = node; // chain `node` to `prev`
    }
    while prev.as_ref().unwrap().next.is_some() {
        prev = &mut prev.as_mut().unwrap().next; // find the reversed tail
    }
    prev.as_mut().unwrap().next = curr; // join the rest chain (from `right` to end) to reversed tail
    dummy.unwrap().next
}