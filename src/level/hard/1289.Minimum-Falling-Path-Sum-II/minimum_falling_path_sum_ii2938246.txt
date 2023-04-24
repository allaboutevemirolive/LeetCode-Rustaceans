// https://leetcode.com/problems/minimum-falling-path-sum-ii/solutions/2938246/rust-dp-fast/
impl Solution {
    pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        if grid.len() == 1 && grid[0].len() == 1 {
            return grid[0][0];
        }
        let mut dp = vec![0;grid[0].len()];
        for i in 0..grid.len() {
            //min index, min value next min value
            let mut mi = 0;
            let mut mv = dp[0];
            let mut mv2 = i32::MAX;
            for i in 1..dp.len() {
                if mv > dp[i] {
                    mv2 = mv;
                    mv = dp[i];
                    mi = i
                } else if mv == dp[i] {
                    mv2 = mv;
                } else if mv2 > dp[i] {
                    mv2  = dp[i]
                }
            }
            //println!("{} {} {}",mi, mv, mv2);
            let mut ndp = vec![0;dp.len()];
            for j in 0..dp.len() {
                if j == mi {
                    ndp[j] = mv2+grid[i][j];
                } else {
                    ndp[j] = mv+grid[i][j];
                }
            }
            dp = ndp;
            //println!("{:?}",dp);
        }
        return *dp.iter().min().unwrap();
    }
}