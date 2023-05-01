// https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/solutions/1127047/rust-no-allocation-iterative-o-n-solution/
pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
	let mut dummy = ListNode { val: 0, next: head, };
	let mut next = &mut dummy.next;
	loop {
		let mut following = None;
		if let Some(curr) = next.as_mut() {
			let curr_val = curr.val;
			let mut curr_cnt = 1;
			{
				let mut inner = curr;
				while inner.next.is_some() {
					let inner_next_val = inner.next.as_mut().unwrap().val;
					if inner_next_val == curr_val {
						curr_cnt += 1;
						inner = inner.next.as_mut().unwrap();
					} else {
						break;
					};
				};
				if curr_cnt > 1 {
					following = Some(inner.next.take());
				};
			};
		} else {
			break;
		};
		if let Some(following) = following {
			*next = following;
		} else {
			next = &mut next.as_mut().unwrap().next;
		};
	};
	dummy.next
}