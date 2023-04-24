// https://leetcode.com/problems/minimum-cost-to-merge-stones/solutions/2745842/rust-easy-to-read-dp-solution/
impl Solution {
    pub fn merge_stones(stones: Vec<i32>, k: i32) -> i32 {
        let (n, k) = (stones.len(), k as usize);
        if (n - 1) % (k - 1) != 0 { return -1 }
        
        let mut sum = vec![0; n + 1];
        let mut dp = vec![vec![vec![-1; k + 1]; n]; n];
        
        for i in 0 .. n {
            sum[i + 1] = stones[i] + sum[i];
            dp[i][i][1] = 0; 
        }
        Self::eval(&mut dp, 0, n - 1, k, &sum, 1)
    }
    
    fn eval(dp: &mut Vec<Vec<Vec<i32>>>, i: usize, j: usize, k: usize, sum: &Vec<i32>, sz: usize) ->i32 {
        if dp[i][j][sz] != -1 { return dp[i][j][sz] }
        
        dp[i][j][sz] = i32::MAX;
        if sz == 1 {
            for it in i .. j {
                let v1 = Self::eval(dp, i, it, k, sum, 1);
                let v2 = Self::eval(dp, it + 1, j, k, sum, k - 1);
                if v1 == i32::MAX || v2 == i32::MAX { continue }
                
                dp[i][j][sz] = dp[i][j][sz].min(v1 + v2); 
            }
            
            if dp[i][j][sz] != i32::MAX { dp[i][j][sz] += sum[j + 1] - sum[i]; }
            
            return dp[i][j][sz]
        }
        
        for it in i .. j {
            let v1 = Self::eval(dp, i, it, k, sum, 1);
            let v2 = Self::eval(dp, it + 1, j, k, sum, sz - 1);
            if v1 == i32::MAX || v2 == i32::MAX { continue } 
                
            dp[i][j][sz] = dp[i][j][sz].min(v1 + v2); 
        }
        
        dp[i][j][sz]
    }
}