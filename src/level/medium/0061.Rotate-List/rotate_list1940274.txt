// https://leetcode.com/problems/rotate-list/solutions/1940274/rust-option-take-method/
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        // use the Option.take() method to achive the result.
        // when use the Option.take(), it will take the content and leave None
		//
		// not quite sure, but i think this method O_time(N), O_space(N)
        // ex. 
        /* 
        if there have a node like this
        node1=ListNode {val, next: {ListNode{...}}}
        
        and run this code
        > node2 = node1.next.take();
        
        then we can get the result like
        node1=ListNode {val, next: None}
        node2=ListNode {val, next: ListNode{...}}
        and move the ownership
        */
        
        // Note. this performace may worse than 
        //      store the element in Vec<i32>
        //      and re-generate the new ListNode to return 
        //  because we need to create a larger space ( Vec<Option<Box<T>>> ) and fat pointer move.
        //
        // We can use below code to check the ListNode address witch wrapped with Box.
        // println!("{:p}", &**head.as_ref().unwrap());
        
        if k==0 {return head;}
        let mut head = head;
        let mut arr: Vec<Option<Box<ListNode>>> = Vec::new();
        
        while head.is_some() {
            let next = head.as_mut().unwrap().next.take();
            arr.push(head);
            head = next;
        }
        
        let length = arr.len();
        if length==0 {return  None;}

        let sp = length-(k as usize%length);
        let mut res = arr[(sp + length -1) % length].take();
        for i in 1..length {
            let mut tmp = arr[(sp + length - i -1) % length].take();
            tmp.as_mut().unwrap().next = res;
            res = tmp;
        }

        res
    }
}