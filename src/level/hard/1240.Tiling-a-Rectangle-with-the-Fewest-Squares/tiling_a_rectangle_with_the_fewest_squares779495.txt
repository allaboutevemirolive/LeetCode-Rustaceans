// https://leetcode.com/problems/tiling-a-rectangle-with-the-fewest-squares/solutions/779495/rust-translated-0ms-2-1m/
//  From https://leetcode.com/problems/tiling-a-rectangle-with-the-fewest-squares/discuss/778475/C%2B%2B-recursion-%2B-memoization-with-comments
impl Solution {
    pub fn tiling_rectangle(mut n: i32, mut m: i32) -> i32 {
        fn dfs(mut n: usize, mut m: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
            if m < n {
                std::mem::swap(&mut n, &mut m);
            }
            if n == 0 {
                return 0;
            }
            if n == 1 {
                return m as i32;
            }
            if memo[n as usize][m as usize] != -1 {
                return memo[n as usize][m as usize];
            };
            let mut ans = std::i32::MAX;
            for i in 1..=n {
                let nn = n - i;
                let mm = m - i;
                ans = std::cmp::min(
                    ans,
                    std::cmp::min(dfs(nn, m, memo) + dfs(i, mm, memo), dfs(nn, i, memo) + dfs(mm, n, memo)) + 1,
                );

                for j in 0..std::cmp::min(nn, i) {
                    ans = std::cmp::min(
                        ans,
                        dfs(nn, i - j, memo) + dfs(nn - j, mm + j, memo) + dfs(mm, i + j, memo) + 2,
                    );
                }

                for j in 1..std::cmp::min(nn, i) {
                    ans = std::cmp::min(
                        ans,
                        dfs(mm, i - j, memo) + dfs(mm - j, nn + j, memo) + dfs(nn, i + j, memo) + 2,
                    );
                }
            }
            memo[n as usize][m as usize] = ans;
            ans
        }

        if m < n {
            std::mem::swap(&mut n, &mut m);
        }
        let mut memo = vec![vec![-1; m as usize + 1]; n as usize + 1];
        dfs(n as usize, m as usize, &mut memo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tiling_rectangle() {
        assert_eq!(Solution::tiling_rectangle(2, 3), 3)
    }

    #[test]
    fn test_tiling_rectangle_02() {
        assert_eq!(Solution::tiling_rectangle(5, 8), 5)
    }

    #[test]
    fn test_tiling_rectangle_03() {
        assert_eq!(Solution::tiling_rectangle(11, 13), 6)
    }
}