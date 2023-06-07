// https://leetcode.com/problems/split-array-largest-sum/solutions/3524913/rust-binary-search-0ms/
impl Solution {
    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let nums_sum = nums.iter().sum();
        if k == 1 {
            return nums_sum;
        }

        let max_num = *nums.iter().max().unwrap();
        if can_split_array(&nums, k, max_num) {
            return max_num;
        }

        let mut low = max_num;
        let mut high = nums_sum;

        while low + 1 < high {
            let mid = low + (high - low) / 2;
            if can_split_array(&nums, k, mid) {
                high = mid;
            } else {
                low = mid;
            }
        }
        high
    }
}

pub fn can_split_array(nums: &[i32], k: usize, max: i32) -> bool {
    let mut running_sum = 0;
    let mut intervals = 1;

    for &num in nums.iter() {
        if running_sum + num > max {
            if intervals < k {
                intervals += 1;
                running_sum = num;
            } else {
                return false;
            }
        } else {
            running_sum += num;
        }
    }
    true
}