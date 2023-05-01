// https://leetcode.com/problems/triangle/solutions/727074/rust-0ms-dp/
impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let level = triangle.len();
        if level < 0 {
            unreachable!()
        };
        let mut dp = triangle[level - 1].to_vec();
        for i in (0..level - 1).rev() {
            for j in 0..i + 1 {
                dp[j] = triangle[i][j] + std::cmp::min(dp[j], dp[j + 1])
            }
        }
        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_total() {
        let triangle = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
        assert_eq!(Solution::minimum_total(triangle), 11)
    }
}
