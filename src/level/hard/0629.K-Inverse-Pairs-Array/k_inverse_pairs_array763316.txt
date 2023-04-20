// https://leetcode.com/problems/k-inverse-pairs-array/solutions/763316/rust-1528-ms/
impl Solution {
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
           let n : usize = n as usize;
    let k : usize = k as usize;
    let mut dp: Vec<Vec<i32>> = vec![vec![0;k+1];n+1];
    for i in 1..=n {
        for j in 0..=k{
            if j ==0 {
                dp[i][j] = 1;
            } else {
                for p in 0..=(std::cmp::min(i-1,j)) {
                    dp[i][j] = (dp[i][j] + dp[i-1][j-p]) % 1000000007;
                } 
            }
        }
    }
    dp[n][k]
    }
}