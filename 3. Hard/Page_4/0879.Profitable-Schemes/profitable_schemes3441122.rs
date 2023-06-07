// https://leetcode.com/problems/profitable-schemes/solutions/3441122/rust-dp-solution/
impl Solution {
    const MOD: i32 = 1e9 as i32 + 7;
    
    pub fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
        let n = n as usize;
        let min_profit = min_profit as usize;

        let mut f = vec![vec![0; min_profit + 1]; n + 1];

        for i in 0..n + 1 {
            f[i][0] = 1;
        }

        let len = group.len();

        for i in 1..len + 1 {
            let g = group[i - 1] as usize;
            let p = profit[i - 1] as usize;

            for j in (g..n + 1).rev().step_by(1) {
                for k in (0..min_profit + 1).rev().step_by(1) {
                    let index = if k >= p { k - p } else { 0 };
                    f[j][k] = (f[j][k] + f[j - g][index]) % Solution::MOD;
                }
            }
        }

        f[n][min_profit]
    }
}