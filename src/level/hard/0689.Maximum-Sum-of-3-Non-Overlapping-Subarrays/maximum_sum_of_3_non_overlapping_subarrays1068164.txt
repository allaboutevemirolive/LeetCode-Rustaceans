// https://leetcode.com/problems/maximum-sum-of-3-non-overlapping-subarrays/solutions/1068164/rust-simple-dp-0ms-2-6mb/
use std::cmp::max;
impl Solution {
    pub fn max_sum_of_three_subarrays(nums: Vec<i32>, kk: i32) -> Vec<i32> {
        let k = kk as usize;
        let n = nums.len();
        let mut ss = vec![0;n + 1];
        for i in 0..n {
            ss[i + 1] = ss[i] + nums[i];
        }
        let mut m = vec![[0;4];n + 1];
        for i in k..(n + 1) {
            let s = ss[i] - ss[i - k];
            for j in 1..4 {
                m[i][j] = max(m[i - 1][j], s + m[i - k][j - 1]);
            }
        }
        let mut ans = vec![0;3];
        let mut i = n;
        for j in (1..4).rev() {
            while 0 < i && m[i][j] == m[i - 1][j] {
                i -= 1;
            }
            ans[j - 1] = (i - k) as i32;
            i -= k;
        }
        ans
    }
}
