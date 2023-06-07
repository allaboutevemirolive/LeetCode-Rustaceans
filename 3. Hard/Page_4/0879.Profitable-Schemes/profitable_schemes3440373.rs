// https://leetcode.com/problems/profitable-schemes/solutions/3440373/rust-elixir-bottom-up-dp/
impl Solution {
    pub fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
        let n = n as usize;
        let m = min_profit as usize;
        let mut dp: Vec<Vec<i32>> = vec![vec![0; n + 1]; m + 1];
        dp[0][0] = 1;
        let large = 1_000_000_007;
        
        for (&g, &p) in group.iter().zip(profit.iter()) {
            let g = g as usize;
            let p = p as usize;
            if g > n {
                continue;
            }
            for i in (0..=n - g).rev() {
                for j in (0..=m).rev() {
                    let i2 = i + g;
                    let j2 = m.min(j + p);
                    dp[j2][i2] = (dp[j2][i2] + dp[j][i]) % large;
                }
            }
        }
        let mut ans = 0;
        for &x in dp[m].iter() {
            ans = (ans + x) % large;
        }
        ans
    }
}