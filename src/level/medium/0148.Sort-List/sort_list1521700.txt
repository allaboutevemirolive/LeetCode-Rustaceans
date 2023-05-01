// https://leetcode.com/problems/sort-list/solutions/1521700/rust-lazy-vec-simple-solution-16-95-5-9-mb/
impl Solution {
    pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut vec: Vec<i32> = vec![];
        while let Some(mut node) = head { 
            vec.push(node.val);
            head = node.next;
        } 
        vec.sort(); 
        let mut next = None; 
        while let Some(val) = vec.pop() { 
            next = Some(Box::new(ListNode{next, val}))
        }
        next 
    }
}