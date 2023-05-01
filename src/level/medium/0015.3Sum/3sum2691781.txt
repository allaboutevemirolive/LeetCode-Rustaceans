// https://leetcode.com/problems/3sum/solutions/2691781/binary-search-rust/
use std::cmp::Ordering;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut res = Vec::new();

        for i in 0..nums.len() {
            if i > 0 && nums[i - 1] == nums[i] { continue; } // skip duplicates

            // Binary search
            let mut lo = i + 1;
            let mut hi = nums.len() - 1;

            while lo < hi {
                match (nums[i] + nums[lo] + nums[hi]).cmp(&0) {
                    Ordering::Greater => hi -= 1,
                    Ordering::Less => lo += 1,
                    Ordering::Equal => {
                        res.push(vec![nums[i], nums[lo], nums[hi]]);
                        lo += 1;
                        hi -= 1;

                        while lo < hi && nums[lo] == nums[lo - 1] { lo += 1 } // skip duplicates
                        while lo < hi && nums[hi] == nums[hi + 1] { hi -= 1 } // skip duplicates 
                    }
                }
            }
        }
        res
    }
}