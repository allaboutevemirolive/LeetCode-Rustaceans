// https://leetcode.com/problems/triangle/solutions/2146196/rust-solution/
impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        if triangle.len() == 1 {
            return triangle[0][0];
        }
        
        let mut dp = triangle.last().unwrap().clone();
        for row in triangle.iter().rev().skip(1) {
            for (i, &num) in row.iter().enumerate() {
                dp[i] = num + dp[i].min(dp[i + 1]);
            }
        }
        dp[0]
    }
}