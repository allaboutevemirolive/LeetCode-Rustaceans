// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/solutions/1875748/rust-0ms-100-faster/
fn min(a: i32, b: i32) -> i32 {
    if a < b {
        return a;
    }

    return b;
}

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let (mut low, mut high) = (0, n - 1);
        let mut res = nums[0];

        while low <= high {
            if nums[low] < nums[high] {
                res = min(res, nums[low]);
                break;
            }

            let mid = low + (high - low) / 2;
            res = min(nums[mid], res);
            if nums[mid] >= nums[low] {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }

        return res;
    }
}