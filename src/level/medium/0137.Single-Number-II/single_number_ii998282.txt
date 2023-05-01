// https://leetcode.com/problems/single-number-ii/solutions/998282/rust-quicksort-solution/
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        fn q_sort(nums: &mut [i32]) {
            if nums.len() <= 1 {
                return;
            }
            
            let mid = nums.len() / 2;
            let (mut left, mut right) = (0, nums.len()-1);
            nums.swap(mid, right);
            
            for i in 0..nums.len() {
                if nums[i] > nums[right] {
                    nums.swap(left, i);
                    left += 1;
                }
            }
            
            nums.swap(left, right);
            q_sort(&mut nums[0..left]);
            q_sort(&mut nums[left+1..=right]);
        }
        
        let mut nums = nums;
        q_sort(&mut nums);
        
        let (mut res, mut i) = (0, 0);
        while i < nums.len() {
            if i+1 == nums.len() || nums[i] != nums[i+1] {
                return nums[i];
            }
            i += 3;
        }
        res
    }
}