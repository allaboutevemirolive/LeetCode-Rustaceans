// https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/solutions/740963/rust-single-loop/
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none()
        {
            return head;
        }
        let mut head = head;
        let mut res = ListNode::new(-1);
        res.next = head;
        let mut cur = &mut res;
        let mut remove = cur.next.as_mut().unwrap().val - 1;
        
        while cur.next.is_some()
        {
            let val = cur.next.as_mut().unwrap().val;
            let mut next_val = val-1;
            if let Some(n) = &cur.next.as_mut().unwrap().next
            {
                next_val = n.val;
            }
            if(val == next_val || val == remove)
            {
                remove = val;
                let next = cur.next.as_mut().unwrap().clone();
                cur.next = next.next;
                continue;
            }
            cur = cur.next.as_mut().unwrap();
        }
        
        return res.next;
    }
}