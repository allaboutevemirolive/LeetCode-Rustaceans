// https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/solutions/960404/rust-use-safe-code/
use std::collections::HashMap;
impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut p = head.as_mut();

        while let Some(cur) = p {
            *map.entry(cur.val).or_insert(0) += 1;
            p = cur.next.as_mut();
        }

        let mut new_head = Box::new(ListNode::new(0));
        new_head.next = head;

        let mut pre = new_head.as_mut();

        while let Some(cur) = pre.next.as_mut() {
            if let Some(v) = map.get(&cur.val) {
                if *v > 1 {
                    pre.next = cur.next.take();
                } else {
                    pre = pre.next.as_mut().unwrap();
                }
            } else {
                pre = pre.next.as_mut().unwrap();
            }
        }

        new_head.next
    }
}
