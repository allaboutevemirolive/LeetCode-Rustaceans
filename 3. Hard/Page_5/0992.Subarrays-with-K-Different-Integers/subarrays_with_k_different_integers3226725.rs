// https://leetcode.com/problems/subarrays-with-k-different-integers/solutions/3226725/rust-implementation/

use std::collections::HashMap;

impl Solution {
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        let mut lookup: HashMap<i32, usize> = HashMap::new();
        let mut result = 0;

        let mut start = 0;
        let mut start_k = 0;
        let k = k as usize;

        for &num in &nums {
            *lookup.entry(num).or_insert(0) += 1;

            if lookup.len() == k + 1 {
                // remove the distinct at start_k, move start_k, start
                lookup.remove(&nums[start_k]);
                start_k += 1;
                start = start_k;
            }

            if lookup.len() == k {
                // update start_k and res (Notice: k >= 1)
                while lookup[&nums[start_k]] > 1 {
                    *lookup.get_mut(&nums[start_k]).unwrap() -= 1;
                    start_k += 1;
                }
                result += start_k - start + 1;
            }
        }

        result as i32
    }

}