// https://leetcode.com/problems/rotate-list/solutions/1497517/rust-0ms-o-n-o-1-space/
type Owned = Box<ListNode>;

// helper fn for building linked list
fn from_to(mut node: Owned, next: Option<Owned>) -> Option<Owned> {
    node.next = next;
    Some(node)
}

// reverse list and get length
fn rev_len(head: Option<Owned>) -> (Option<Owned>, i32) {
    (0..).scan(head, |head, _| head.take()
            .map(|mut pop| { *head = pop.next.take(); pop }))
        .fuse()
        .fold((None, 0), |(node, len), pop| (from_to(pop, node), len + 1))
}

impl Solution {
fn rotate_right(head: Option<Owned>, k: i32) -> Option<Owned> {
    // reverse list
    let (mut head, len) = rev_len(head);
    if len <= 1 { return head; }

    // chop off the amount we need to rotate by
    let mut ans = (0..k % len)
        .fold(&mut head, |p, _| &mut p.as_deref_mut().unwrap().next)
        .take();

    // append the rest to chopped portion
    let mut cur = &mut ans;
    while cur.is_some() { cur = &mut cur.as_deref_mut().unwrap().next; }
    *cur = head;

    // reverse again
    rev_len(ans).0
}
}