// https://leetcode.com/problems/number-of-ways-of-cutting-a-pizza/solutions/3361878/rust-solution/
impl Solution {
    pub fn ways(pizza: Vec<String>, k: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;

        let n = pizza.len();
        let m = pizza[0].len();

        let mut grid = vec![vec![0; m + 1]; n + 1];
        for (i, row) in pizza.into_iter().enumerate() {
            for (j, x) in row.into_bytes().into_iter().enumerate() {
                grid[i + 1][j + 1] =
                    grid[i + 1][j] + grid[i][j + 1] - grid[i][j] + (x == b'A') as i32;
            }
        }

        let get = |i1: usize, j1: usize, i2: usize, j2: usize| {
            grid[i2][j2] + grid[i1][j1] > grid[i2][j1] + grid[i1][j2]
        };

        let mut d = vec![vec![0; m]; n];
        for i in 0..n {
            for j in 0..m {
                d[i][j] = get(i, j, n, m) as _;
            }
        }
        for _ in 1..k {
            for i in 0..n {
                for j in 0..m {
                    d[i][j] = (i + 1..n)
                        .filter_map(|ni| get(i, j, ni, m).then(|| d[ni][j]))
                        .chain((j + 1..m).filter_map(|nj| get(i, j, n, nj).then(|| d[i][nj])))
                        .fold(0, |a, x| (a + x) % MOD);
                }
            }
        }
        d[0][0]
    }
}
