// https://leetcode.com/problems/stone-game-iii/solutions/645913/rust-top-down-dp-runs-in-28ms/
impl Solution {
    pub fn stone_game_iii(stones: Vec<i32>) -> String {
        let n = stones.len();
        let mut dp: Vec<i32> = vec![0; n + 1];
        
        for idx in (0..n).rev() {
            let mut sum = 0;
            dp[idx] = i32::min_value();
            
            for step in 0..3 {
                if idx + step >= n {
                    break;
                }

                sum += stones[idx + step];
                dp[idx] = dp[idx].max(sum - dp[idx + step + 1]);
            }
        }
        
        match dp[0] {
            d if d == 0 => "Tie".to_owned(),
            d if d < 0 => "Bob".to_owned(),
            _ => "Alice".to_owned()
        }
    }
}