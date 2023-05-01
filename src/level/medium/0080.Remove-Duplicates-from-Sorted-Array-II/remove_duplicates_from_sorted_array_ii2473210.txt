// https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/solutions/2473210/rust-solution/
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
	let mut len = nums.len();
	let mut i = 2;

	while i < len {
		if nums[i - 2] == nums[i - 1] && nums[i - 1] == nums[i] {
			nums.remove(i);
			len -= 1;
			continue;
		}

		i += 1;
	}

	nums.len() as _
}