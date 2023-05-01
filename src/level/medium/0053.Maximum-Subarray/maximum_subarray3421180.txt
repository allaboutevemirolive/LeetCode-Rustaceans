// https://leetcode.com/problems/maximum-subarray/solutions/3421180/rust-solution/
use std::cmp::max;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        // let mut maxi = i32::MIN;
        // let mut sum = 0;
        // for num in nums {
        //     sum += num;
        //     if(num > sum) {
        //         sum = num;
        //     }
        //     maxi = max(sum,maxi);
        // }
        // maxi
        let mut arr = vec![0; nums.len()];
        arr[0] = nums[0];
        for i in 1..nums.len() {
            arr[i] = max(nums[i]+arr[i-1], nums[i]);
        }
        *arr.iter().max().unwrap()
    }
}