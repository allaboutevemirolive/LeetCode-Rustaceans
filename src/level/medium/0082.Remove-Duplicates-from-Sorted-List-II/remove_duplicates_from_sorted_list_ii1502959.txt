// https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/solutions/1502959/simple-rust-solution/
pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = match head.take() {
        None => return None,
        Some(mut node) => {
            head = node.next.take();
            node
        }
    };

    let mut dummy = ListNode::new(0);
    let mut tail = &mut dummy.next;
    let mut skip = false;

    while let Some(mut node) = head.take() {
        head = node.next.take();

        if node.val == prev.val {
            skip = true;
            continue;
        }

        if skip {
            skip = false;
            prev = node;
        } else {
            std::mem::swap(&mut prev, &mut node);
            *tail = Some(node);
            tail = &mut tail.as_mut().unwrap().next;
        }
    }

    if !skip {
        *tail = Some(prev);
    }

    dummy.next.take()
}