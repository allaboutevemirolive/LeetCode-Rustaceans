// https://leetcode.com/problems/dice-roll-simulation/solutions/2779607/rust-dp-solution/
impl Solution {
    pub fn die_simulator(n: i32, roll_max: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut dp = vec![[1i64; 6]; n];
        
        for i in 1 .. n {
            for j in 0 .. 6 {
                dp[i][j] = 0;
                for k in 0 .. 6 { dp[i][j] += dp[i - 1][k]; }
                
                if roll_max[j] as usize <= i { 
                
                    if roll_max[j] as usize == i { dp[i][j] -= 1; } 
                    else {
                        for k in 0 .. 6 {
                            if k != j { dp[i][j] -= dp[i - 1 - roll_max[j] as usize][k]; }
                        }
                    }
                }
                dp[i][j] %= 1_000_000_007;
                if dp[i][j] < 0 { dp[i][j] += 1_000_000_007; }
            }
        }
        
        (dp[n - 1].iter().sum::<i64>() % 1_000_000_007) as _
    }
}
