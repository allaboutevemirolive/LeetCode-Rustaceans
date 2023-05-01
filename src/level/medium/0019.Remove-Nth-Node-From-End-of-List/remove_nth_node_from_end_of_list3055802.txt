// https://leetcode.com/problems/remove-nth-node-from-end-of-list/solutions/3055802/rust-simple-solution/
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut nodes: Vec<Box<ListNode>> = Vec::new();
        let mut head = head;

        while let Some(mut node) = head {
            head = node.next.take();
            nodes.push(node);
        }

        nodes.remove((nodes.len() as i32 - n) as usize);
        let mut head = Box::new(ListNode::new(0));

        let mut node_pointer = head.as_mut();

        for node in nodes.into_iter() {
            node_pointer.next = Some(node);
            node_pointer = node_pointer.next.as_mut().take().unwrap();
        }
        head.next
    }
}