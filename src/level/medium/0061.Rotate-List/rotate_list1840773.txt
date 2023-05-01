// https://leetcode.com/problems/rotate-list/solutions/1840773/rust-shorter-solution-0ms-2mb/
type Node = Option<Box<ListNode>>;
impl Solution {
    pub fn rotate_right(mut head: Node, k: i32) -> Node {
        let (mut v, mut res) = (vec![], None);
        if head.is_none() || k <= 0 { return head }
        while let Some(n) = head {
            v.push(n.val);
            head = n.next;
        }
        let j = k as usize % v.len();
        v.rotate_right(j);
        while let Some(val) = v.pop() { 
            res = Some(Box::new(ListNode { next: res, val}))
        }
        res        
    }
}