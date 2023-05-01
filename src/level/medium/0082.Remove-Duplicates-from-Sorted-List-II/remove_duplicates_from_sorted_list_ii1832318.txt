// https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/solutions/1832318/rust-recursive-and-matching-patterns/
pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
	match head {
		None => None,
		Some(mut n) => {
			match n.next {
				None => Some(n),
				Some(n2) if n2.val != n.val => {
					n.next = Self::delete_duplicates(Some(n2));
					Some(n)
				}
				Some(mut n2) => { // n is a dupe, discard n and find the next node with different value
					while n2.val == n.val {
						match n2.next {
							None => return None,
							Some(n3) => n2 = n3,
						};
					}
					Self::delete_duplicates(Some(n2))
				}
			}
		}
	}
}