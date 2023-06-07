// https://leetcode.com/problems/number-of-ways-of-cutting-a-pizza/solutions/3362136/rust-dp-solution/
const MOD: i32 = 1e9 as i32 + 7;

impl Solution {
    pub fn ways(pizza: Vec<String>, k: i32) -> i32 {
        let (m, n) = (pizza.len(), pizza[0].len());
        let k = k as usize;
        let mut f: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; k + 1]; n + 1]; m + 1];
        let mut pre_sum = vec![vec![0; n + 1]; m + 1];

        for i in (0..m).rev() {
            let chars: Vec<char> = pizza[i].chars().collect();
            for j in (0..n).rev() {
                pre_sum[i][j] = if chars[j] == 'A' { 1 } else { 0 };
                pre_sum[i][j] += pre_sum[i][j + 1] + pre_sum[i + 1][j] - pre_sum[i + 1][j + 1];
            }
        }

        for i in (0..m).rev() {
            for j in (0..n).rev() {
                if pre_sum[i][j] > 0 {
                    f[i][j][1] = 1;
                }

                for p in 2..k + 1 {
                    // horizontal cut
                    for x in (1..m - i).rev() {
                        if pre_sum[i][j] - pre_sum[i + x][j] > 0 {
                            f[i][j][p] = (f[i][j][p] + f[i + x][j][p - 1]) % MOD;
                        }
                    }

                    // vertical cut
                    for y in (1..n - j).rev() {
                        if pre_sum[i][j] - pre_sum[i][j + y] > 0 {
                            f[i][j][p] = (f[i][j][p] + f[i][j + y][p - 1]) % MOD;
                        }
                    }
                }
            }
        }

        f[0][0][k]
    }
}