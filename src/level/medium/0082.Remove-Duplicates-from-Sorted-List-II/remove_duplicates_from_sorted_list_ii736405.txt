// https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/solutions/736405/clean-rust-solution/
pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode {
        val: 0,
        next: head,
    });

    fn recursive_remove(head: &mut Box<ListNode>) {
        if let Some(mut reference) = head.next.take() {
            let mut count = 0;
            while let Some(next) = reference.next.take() {
                if next.val == reference.val {
                    reference = next;
                    count += 1;
                } else {
                    reference.next = Some(next);
                    break;
                }
            }

            recursive_remove(&mut reference);
            head.next = if count == 0 {
                Some(reference)
            } else {
                reference.next
            };
        }
    }

    recursive_remove(&mut dummy);
    dummy.next
}