// https://leetcode.com/problems/minimum-distance-to-type-a-word-using-two-fingers/solutions/764964/rust-0ms/
impl Solution {
    pub fn minimum_distance(word: String) -> i32 {
         let mut keys: Vec<usize> = vec![0; word.len()];
    for (i,bytes) in word.bytes().enumerate() {
        keys[i] = bytes as usize - 'A' as usize;
    }
    let mut cost:Vec<Vec<i32>> = vec![vec![0;26];27];
    for i in 0..26{
        for j in i..26{
            cost[i][j] = (i as i32/6 - j as i32/6).abs() + (i as i32%6 - j as i32%6).abs();
            cost[j][i] = cost[i][j];
        }
    }
    let mut dp:Vec<i32> = vec![0;27];
    for i in (1..keys.len()).rev() {
        let dp_at_minus_1 = dp[keys[i-1]];
        let cost_at_minus_1_and_i = cost[keys[i-1]][keys[i]];
        for j in 0..27{
            dp[j] = std::cmp::min(dp_at_minus_1 + cost[j][keys[i]], dp[j] + cost_at_minus_1_and_i);
        }
    }
    dp[26]
    }
}