// https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/solutions/1412615/rust-solution-for-both-26-and-80/
// For problem 80, it's 2
// For problem 26, it's 1
const most_appear_times: usize = 2;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return nums.len() as i32;
        }

        let mut tail = most_appear_times;
        for i in most_appear_times..nums.len() {
            if nums[i] != nums[tail - most_appear_times] {
                nums[tail] = nums[i];
                tail += 1;
            }
        }

        // optional
        // nums.truncate(tail);

        tail as i32
    }
}
