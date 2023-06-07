// https://leetcode.com/problems/cherry-pickup/solutions/817454/rust-translated-0ms-100/
impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut dp = vec![vec![std::i32::MIN; n]; n];
        dp[0][0] = grid[0][0];

        for t in 1..2 * n - 1 {
            let mut dp2 = vec![vec![std::i32::MIN; n]; n];
            let (r1, r2) = if n - 1 < t {
                (t + 1 - n, n - 1)
            } else {
                (0, t)
            };
            for i in r1..=r2 {
                for j in r1..=r2 {
                    if grid[i][t - i] == -1 || grid[j][t - j] == -1 {
                        continue;
                    }
                    let mut val = grid[i][t - i];
                    if i != j {
                        val += grid[j][t - j];
                    }
                    for pi in i as i32 - 1..=i as i32 {
                        for pj in j as i32 - 1..=j as i32 {
                            if pi >= 0 && pj >= 0 {
                                dp2[i][j] =
                                    std::cmp::max(dp2[i][j], dp[pi as usize][pj as usize] + val);
                            }
                        }
                    }
                }
            }
            dp = dp2;
        }
        std::cmp::max(0, dp[n - 1][n - 1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cherry_pickup() {
        assert_eq!(
            Solution::cherry_pickup(vec![vec![0, 1, -1], vec![1, 0, -1], vec![1, 1, 1]]),
            5
        )
    }
}