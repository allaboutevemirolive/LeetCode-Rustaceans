// https://leetcode.com/problems/count-vowels-permutation/solutions/2390072/rust-with-a-closure/
impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        let mut dp = vec![1; 5];
        let add = |a, b| (a + b) % 1_000_000_007;
        for _ in 1..n {
            let mut dp2 = Vec::with_capacity(5);
            dp2.push(dp[1]);
            dp2.push(add(dp[0], dp[2]));
            dp2.push(add(dp[0], add(dp[1], add(dp[3], dp[4]))));
            dp2.push(add(dp[2], dp[4]));
            dp2.push(dp[0]);
            dp = dp2;
        }
        dp.into_iter().fold(0, add)
    }
}