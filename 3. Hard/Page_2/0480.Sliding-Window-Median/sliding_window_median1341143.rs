// https://leetcode.com/problems/sliding-window-median/solutions/1341143/rust-2-heaps-solution-beat-100-mem-speed/
impl Solution {
	pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
		use std::collections::BinaryHeap; // MaxHeap
		use std::cmp::Reverse; // for MinHeap

		// remove an element and rebuild heap
		// only remove once!
		fn rebuild_lhs_without(lhs: BinaryHeap<i32>, x: i32) -> BinaryHeap<i32> {
			let mut heap = BinaryHeap::<i32>::new();
			let mut removed_once = false;
			for c in lhs.into_sorted_vec() {
				if c == x && !removed_once { removed_once = true; }
				else { heap.push(c); }
			};
			heap
		}
		fn rebuild_rhs_without(rhs: BinaryHeap<Reverse<i32>>, x: i32) -> BinaryHeap<Reverse<i32>> {
			let mut heap = BinaryHeap::<Reverse<i32>>::new();
			let mut removed_once = false;
			for c in rhs.into_sorted_vec() {
				if c.0 == x && !removed_once { removed_once = true; }
				else { heap.push(c); }
			};
			heap
		}
		// rhs always has 1 more element
		fn balance(lhs: &mut BinaryHeap<i32>, rhs: &mut BinaryHeap<Reverse<i32>>) {
			let s1 = lhs.len() as i32;
			let s2 = rhs.len() as i32;
			if s1 - s2 >= 1 { rhs.push(Reverse(lhs.pop().unwrap())); }
			if s2 - s1 > 1 { lhs.push(rhs.pop().unwrap().0); }
		}

		let mut lhs = BinaryHeap::<i32>::new();
		let mut rhs = BinaryHeap::<Reverse<i32>>::new();

		let mut res: Vec<f64> = vec!();
		// insert all numbers and balance them
		for (ix, i) in nums.iter().enumerate() {
			// which side should the num insert?
			if rhs.is_empty() || (!rhs.is_empty() && rhs.peek().unwrap().0 < *i) { rhs.push(Reverse(*i)); }
			else { lhs.push(*i); }

			balance(&mut lhs, &mut rhs);

			// remove oldest elem
			let total = (rhs.len() + lhs.len()) as i32;
			if total > k {
				let oldest = &nums[ix - k as usize];
				// remove oldest
				if oldest >= &rhs.peek().unwrap().0 { rhs = rebuild_rhs_without(rhs, *oldest); }
				else { lhs = rebuild_lhs_without(lhs, *oldest); }

				balance(&mut lhs, &mut rhs);
			}
			// we return the median!
			if total >= k {
				let median = if k % 2 == 1 {
					rhs.peek().unwrap().0 as f64
				} else {
					(*lhs.peek().unwrap()) as f64 * 0.5f64 + rhs.peek().unwrap().0 as f64 * 0.5f64
				};
				res.push(median);
			}
		}
		res
	}
}