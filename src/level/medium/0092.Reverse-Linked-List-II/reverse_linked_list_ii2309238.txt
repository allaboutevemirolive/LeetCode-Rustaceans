// https://leetcode.com/problems/reverse-linked-list-ii/solutions/2309238/rust-simple-vector-solution/
impl Solution {
    pub fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut data = vec![];
        while let Some(mut node) = head {
            head = node.next;
            node.next = None;
            data.push(node);
        }
        
        let (i, j) = (left as usize - 1, right as usize - 1);
        data[i..=j].reverse();
        
        let mut ret = None;
        let mut tail = &mut ret;
        for p in data { tail = &mut tail.insert(p).next; }
        
        ret
    }
}