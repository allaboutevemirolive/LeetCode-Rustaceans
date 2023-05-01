// https://leetcode.com/problems/reverse-linked-list-ii/solutions/2311334/rust-100-unsafe-block-o-n/
impl Solution {
  pub fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
	  let mut dummy = Box::new(ListNode::new(0));
	  dummy.next = head;
	  let mut dummy = Some(dummy);
	  let mut cur = dummy.as_mut();
	  for _ in 0..left-1 {
		if let Some(x) = cur{
		  cur = x.next.as_mut();
		}
	  }
	  let mut before_rev = cur.unwrap();
	  let mut rev_head = before_rev.next.take();
	  let next = Self::reverse(rev_head, right-left+1);
	  before_rev.next = next;
	  dummy.unwrap().next.take()
  }
  fn reverse(head: Option<Box<ListNode>>, count: i32) -> Option<Box<ListNode>>{
	let tail = Box::into_raw(head.unwrap());
	let mut pre = None;
	let mut cur = Some(unsafe {Box::from_raw(tail)});
	for _ in 0..count {
	  let mut x = cur.unwrap();
	  cur = x.next;
	  x.next = pre;
	  pre = Some(x);
	}
	unsafe{(*tail).next = cur};
	pre
  }
}