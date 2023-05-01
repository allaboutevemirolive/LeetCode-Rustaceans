// https://leetcode.com/problems/single-number-ii/solutions/1587227/rust/
fn single_number(mut nums: Vec<i32>) -> i32 {
	nums.sort();
	if nums.len() < 3 || nums[0] != nums[1] {
		return nums[0];
	}  
	for i in 1..nums.len() - 1 {
		if nums[i - 1] != nums[i] && nums[i] != nums[i + 1] {
			return nums[i];
		}
	}
	nums[nums.len() - 1]
}