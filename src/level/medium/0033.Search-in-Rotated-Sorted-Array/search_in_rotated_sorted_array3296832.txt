// https://leetcode.com/problems/search-in-rotated-sorted-array/solutions/3296832/rust-2-approaches/
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        fn two_bin_search(nums: Vec<i32>, target: i32) -> i32 {
            fn find_pivot(nums: &Vec<i32>) -> usize {
                let mut lo = 0;
                let mut hi = nums.len() - 1;
                while lo < hi {
                    let mid = lo + (hi - lo) / 2;
                    if nums[mid] > nums[hi] {
                        lo = mid + 1;
                    } else {
                        hi = mid;
                    }
                }
                lo
            }
            fn bin_search(nums: &Vec<i32>, target: i32, rot: usize) -> i32 {
                let mut lo = 0_i32;
                let mut hi = nums.len() as i32 - 1;

                while lo <= hi {
                    let mid = lo + (hi - lo) / 2;
                    let real_mid = (mid as usize + rot) % nums.len();
                    if nums[real_mid] < target {
                        lo = mid + 1;
                    } else if nums[real_mid] > target {
                        hi = mid - 1;
                    } else {
                        return real_mid as i32;
                    }
                }
                -1
            }
            let rot = find_pivot(&nums);
            bin_search(&nums, target, rot)
        }
        
        fn one_bin_search(nums: Vec<i32>, target: i32) -> i32 {
            let mut lo = 0;
            let mut hi = nums.len() - 1;

            while lo <= hi {
                let mid = lo + (hi - lo) / 2;

                let mut mid_element = nums[mid];

                let mid_in_second = mid_element < nums[0]; // false -> left arr, true -> right arr
                let target_in_second = target < nums[0]; // false -> left arr, true -> right arr

                if mid_in_second ^ target_in_second {
                    // mid and target in different arrays
                    if target_in_second {
                        mid_element = i32::MIN;
                    } else {
                        mid_element = i32::MAX;
                    }
                }

                if mid_element == target {
                    return mid as i32;
                } else if mid_element < target {
                    lo = mid + 1;
                } else {
                    hi = mid - 1;
                }
            }
            -1
        }
        one_bin_search(nums, target)
    }
}