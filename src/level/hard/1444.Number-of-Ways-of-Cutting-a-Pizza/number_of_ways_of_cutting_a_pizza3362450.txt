// https://leetcode.com/problems/number-of-ways-of-cutting-a-pizza/solutions/3362450/rust-prefix-sum-and-memoization-solution/
use std::collections::HashMap;

impl Solution {
    pub fn ways(pizza: Vec<String>, k: i32) -> i32 {
        fn aux(
            x0: usize,
            y0: usize,
            rest: i32,
            sum: &[Vec<i32>],
            cache: &mut HashMap<(usize, usize, i32), i32>,
        ) -> i32 {
            let x1 = sum.len() - 1;
            let y1 = sum[0].len() - 1;
            if rest == 0 {
                return if sum[x1][y1] - sum[x1][y0 - 1] - sum[x0 - 1][y1] + sum[x0 - 1][y0 - 1] > 0
                {
                    1
                } else {
                    0
                };
            }
            if let Some(&calculated) = cache.get(&(x0, y0, rest)) {
                return calculated;
            }
            let mut count = 0;
            for i in x0..x1 {
                if sum[i][y1] - sum[x0 - 1][y1] - sum[i][y0 - 1] + sum[x0 - 1][y0 - 1] > 0 {
                    count += aux(i + 1, y0, rest - 1, sum, cache);
                    count %= 1000000007;
                }
            }
            for j in y0..y1 {
                if sum[x1][j] - sum[x1][y0 - 1] - sum[x0 - 1][j] + sum[x0 - 1][y0 - 1] > 0 {
                    count += aux(x0, j + 1, rest - 1, sum, cache);
                    count %= 1000000007;
                }
            }
            cache.insert((x0, y0, rest), count);
            count
        }
        let m = pizza.len();
        let n = pizza[0].len();
        let mut sum = vec![vec![0; n + 1]; m + 1];
        for i in 0..m {
            for j in 0..n {
                sum[i + 1][j + 1] = sum[i][j + 1] + sum[i + 1][j] - sum[i][j];
                if pizza[i].as_bytes()[j] == b'A' {
                    sum[i + 1][j + 1] += 1;
                }
            }
        }
        aux(1, 1, k - 1, &sum, &mut HashMap::new())
    }
}