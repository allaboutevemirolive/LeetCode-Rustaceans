// https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/solutions/2681450/rust-layman-s-approach-with-hashmap-and-recursively-build/
use std::collections::HashMap;

impl Solution {
	pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
		let mut map = HashMap::new();
		Self::delete_build(head, &mut map)
	}

	pub fn delete_build(mut node: Option<Box<ListNode>>, map: &mut HashMap<i32, bool>) -> Option<Box<ListNode>> {
		let mut to_build = false;
		while let Some(curr_node) = node.take() {
			if let Some(next_node) = curr_node.next.as_ref() {
				// if current node's value is not equal to next ndoe's value
				// and it is not exist in duplicated value's hashmap then proceed to add it
				if curr_node.val != next_node.val && !map.contains_key(&curr_node.val) {
					to_build = true;
				} else {
					map.insert(curr_node.val, true);
				}
			} else {
				// reach the end here :)
				// this time the next node is `None`
				if !map.contains_key(&curr_node.val) {
					to_build = true;
				}
			}
			if to_build {
				// recursively build the new list
				return Some(Box::new(ListNode {
					val: curr_node.val,
					next: Self::delete_build(curr_node.next, map)
				}));
			} else {
				// pass to the next node
				node = curr_node.next;
			}
		}
		None
	}
}