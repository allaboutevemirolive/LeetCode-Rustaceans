// https://leetcode.com/problems/profitable-schemes/solutions/3440543/rust-100-runtime-and-memory-20-lines/
type M = [[[i32; 101]; 101]; 101];

const MOD: i32 = 10_i32.pow(9) + 7;

impl Solution {
    pub fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut m = [[[-1; 101]; 101]; 101];
        m[group.len()].iter_mut().for_each(|v| v.fill(0));
        m[group.len()].iter_mut().for_each(|v| v[0] = 1);

        fn f(n: i32, min_prof: i32, i: i32, group: &[i32], profit: &[i32], m: &mut M) -> i32 {
            if m[i as usize][n as usize][min_prof as usize] == -1 {
                let mut v = f(n, min_prof, i + 1, group, profit, m);
                if n >= group[i as usize] {
                    let min_profit = (min_prof - profit[i as usize]).max(0);
                    v += f(n - group[i as usize], min_profit, i + 1, group, profit, m);
                }
                m[i as usize][n as usize][min_prof as usize] = v % MOD;
            }
            m[i as usize][n as usize][min_prof as usize]
        }

        f(n, min_profit, 0, &group, &profit, &mut m)
    }
}
