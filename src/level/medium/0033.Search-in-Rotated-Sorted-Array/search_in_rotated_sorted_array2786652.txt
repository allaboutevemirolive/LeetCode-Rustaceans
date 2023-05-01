// https://leetcode.com/problems/search-in-rotated-sorted-array/solutions/2786652/rust-0ms-solution/
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let (mut low, mut high) = (0, n - 1);

        while low <= high {
            let mid = low + (high - low) / 2;
            if nums[mid] == target {
                return mid as i32;
            }
            
            match nums[mid] >= nums[low] {
                true => match target >= nums[low] && target < nums[mid] {
                    true => {
                        high = mid - 1;
                    }
                    false => {
                        low = mid + 1;
                    }
                },
                false => match target <= nums[high] && target > nums[mid] {
                    true => {
                        low = mid + 1;
                    }
                    false => {
                        high = mid - 1;
                    }
                },
            }
        }
        -1
    }
}