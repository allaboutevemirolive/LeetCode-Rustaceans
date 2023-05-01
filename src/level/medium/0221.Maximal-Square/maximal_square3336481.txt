// https://leetcode.com/problems/maximal-square/solutions/3336481/rust-3-approaches/
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        fn using_brute_force(matrix: Vec<Vec<char>>) -> i32 {
            let mut max_sqrt_len = 0;
            let rows = matrix.len();
            let cols = matrix[0].len();
            for i in 0..rows {
                for j in 0..cols {
                    if matrix[i][j] == '1' {
                        let mut sqrt_len = 1;
                        let mut stop = false;
                        while sqrt_len + i < rows && sqrt_len + j < cols && !stop {
                            for k in i..=i + sqrt_len {
                                for h in j..=j + sqrt_len {
                                    if matrix[k][h] == '0' {
                                        stop = true;
                                        break;
                                    }
                                }
                            }

                            if !stop {
                                sqrt_len += 1;
                            }
                        }
                        max_sqrt_len = max_sqrt_len.max(sqrt_len as i32);
                    }
                }
            }
            max_sqrt_len * max_sqrt_len
        }
        fn using_dp_i(matrix: Vec<Vec<char>>) -> i32 {
            let mut max_sqrt_len = 0;
            let rows = matrix.len();
            let cols = matrix[0].len();
            let mut dp = vec![vec![0; cols + 1]; rows + 1];
            for i in 1..=rows {
                for j in 1..=cols {
                    if matrix[i - 1][j - 1] == '1' {
                        dp[i][j] = dp[i][j - 1].min(dp[i - 1][j]).min(dp[i - 1][j - 1]) + 1;
                        max_sqrt_len = max_sqrt_len.max(dp[i][j]);
                    }
                }
            }
            max_sqrt_len * max_sqrt_len
        }
        fn using_dp_ii(matrix: Vec<Vec<char>>) -> i32 {
            let mut max_sqrt_len = 0;
            let rows = matrix.len();
            let cols = matrix[0].len();
            let mut dp = vec![0; cols + 1];
            let mut prev = 0;
            for i in 1..=rows {
                for j in 1..=cols {
                    let temp = dp[j];
                    if matrix[i - 1][j - 1] == '1' {
                        dp[j] = dp[j].min(dp[j - 1].min(prev)) + 1;
                        max_sqrt_len = max_sqrt_len.max(dp[j] as i32);
                    } else {
                        dp[j] = 0;
                    }
                    prev = temp;
                }
            }
            max_sqrt_len * max_sqrt_len
        }
        using_dp_ii(matrix)  
    }
}