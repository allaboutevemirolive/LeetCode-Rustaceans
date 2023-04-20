// https://leetcode.com/problems/maximum-sum-of-3-non-overlapping-subarrays/solutions/647362/rust-clean-o-n-solution-runs-in-4ms-and-with-3-3mb-usage/
use std::iter::repeat;

const SUBARRAYS_COUNT: usize = 3;

impl Solution {
    pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let prefix_sum = build_prefix_sum(&nums);
        let dp = build_dp(&prefix_sum, k as usize, SUBARRAYS_COUNT);
        build_result(&dp, k as usize)
    }
}

fn build_prefix_sum(nums: &[i32]) -> Vec<i32> {
    repeat(&0)
        .take(1)
        .chain(nums.iter())
        .scan(0, |acc, num| {
            *acc += num;
            Some(*acc)
        })
        .collect()
}

fn build_dp(prefix_sums: &[i32], k: usize, levels: usize) -> Vec<Vec<i32>> {
    let mut dp: Vec<Vec<i32>> = vec![vec![0; levels + 1]; prefix_sums.len()];

    for idx in k..dp.len() {
        for j in 1..=levels {
            let cur = dp[idx - k][j - 1] + prefix_sums[idx] - prefix_sums[idx - k];
            let prev = dp[idx - 1][j];
            dp[idx][j] = cur.max(prev);
        }
    }

    dp
}

fn build_result(dp: &Vec<Vec<i32>>, k: usize) -> Vec<i32> {
    let mut result: Vec<i32> = vec![0, 0, 0];
    let mut idx = dp.len() - 1;

    for j in (1..4).rev() {
        while dp[idx][j] == dp[idx - 1][j] {
            idx -= 1;
        }

        idx -= k;
        result[j - 1] = idx as i32;
    }

    result
}