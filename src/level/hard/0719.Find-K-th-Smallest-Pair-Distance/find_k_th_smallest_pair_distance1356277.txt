// https://leetcode.com/problems/find-k-th-smallest-pair-distance/solutions/1356277/rust-binary-search/
impl Solution {
    pub fn smallest_distance_pair(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let n: usize = nums.len();

        // range of possible results
        let mut low: i32 = 0;
        let mut high: i32 = nums[n - 1] - nums[0];

        while low < high {
            // binary search within the range
            let m: i32 = (low + high) / 2;
            let mut count: usize = 0;

            // compute the number of pair distances that is lower than middle
            let mut i: usize = 0;
            for j in 0..n {
                while nums[j] - nums[i] > m {
                    i += 1;
                }
                count += j - i;
            }

            if count < k as usize {
                low = m + 1;
            } else {
                high = m;
            }
        }

        low
    }
}