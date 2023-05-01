// https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/solutions/587742/rust-recursive-solution/
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }

        let mut curr = head;
        let mut next = curr.as_mut().unwrap().next.take();
        let mut curr_val = curr.as_ref().unwrap().val;
        let mut next_val = next.as_ref().unwrap().val;
        match next_val == curr_val {
            true => {
                while next.is_some() && curr.as_ref().unwrap().val == next.as_ref().unwrap().val {
                    curr = next;
                    next = curr.as_mut().unwrap().next.take();
                }
                Self::delete_duplicates(next)
            },
            false => {
                curr.as_mut().unwrap().next = Self::delete_duplicates(next);
                curr
            }
        }
    }
}