// https://leetcode.com/problems/reverse-linked-list-ii/solutions/2314924/rust-solution/
pub fn reverse_and_append(head: Option<Box<ListNode>>, count: i32) -> Option<Box<ListNode>> {
        
        let mut prv = None;
        let mut cur = head.unwrap();
        let mut nxt = Box::new(ListNode::new(0));
        let mut append = None;

        for i in 0..count {
            if let Some(mut nxt) = cur.next {
                cur.next = prv;
                prv = Some(cur);
                if i == count - 1 {
                    append = nxt.next.take();
                }
                cur = nxt;
            }
        }
        cur.next = prv;
    
        let mut cur = Some(cur);
        let mut runner = &mut cur;

        while let Some(node) = runner {
            runner = &mut node.next;
        }
        *runner = append;

        return cur
}

impl Solution {
    pub fn reverse_between(mut head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
        
        if left == right {
            return head
        }
        
        let mut runner = &mut head;
        
        for _ in 1..left {
            if let Some(node) = runner {
                runner = &mut node.next;
            }
        }
        
        *runner = reverse_and_append(runner.take(), right - left);
        
        return head;
    }
}