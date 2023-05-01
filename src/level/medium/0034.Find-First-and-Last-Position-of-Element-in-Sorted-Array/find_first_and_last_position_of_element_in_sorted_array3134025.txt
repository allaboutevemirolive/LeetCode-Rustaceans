// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/solutions/3134025/rust-solution/
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let a = Solution::f(&nums, target);
        if nums.len() == 0 || a >= nums.len() || nums[a] != target {
            vec![-1, -1]
        } else {
            vec![a as i32, Solution::l(&nums, target) as i32]
        }
    }
    
    fn f(nums: &Vec<i32>, target: i32) -> usize {
        let (mut start, mut end) = (0, nums.len());
        while start < end {
            let mid = (start + end) / 2;
            if nums[mid] < target {
                start = mid + 1
            } else {
                end = mid
            }
        }
        return start
    }

    fn l(nums: &Vec<i32>, target: i32) -> usize {
        let (mut start, mut end) = (0, nums.len());
        while start < end {
            let mid = (start + end) / 2;
            if nums[mid] <= target {
                start = mid + 1;
            } else {
                end = mid;
            }
        }
        return start-1
    }
}