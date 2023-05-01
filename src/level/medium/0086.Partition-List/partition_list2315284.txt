// https://leetcode.com/problems/partition-list/solutions/2315284/rust-0ms/
impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut lo = Vec::new();
        let mut hi = Vec::new();
        let mut node = &head;
        
        while let Some(n) = node {
            if n.val < x {
                lo.push(n.val);                
            } else {
                hi.push(n.val);
            }            
            node = &n.next;
        }
        
        let mut ret = None;
        hi.iter().rev().chain(lo.iter().rev()).for_each(|&v| {
            let mut new_node = ListNode::new(v);
            new_node.next = ret.take();
            ret = Some(Box::new(new_node));
        });
        ret
    }
}