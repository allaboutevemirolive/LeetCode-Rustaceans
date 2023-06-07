// https://leetcode.com/problems/stone-game-iii/solutions/3568034/rust-simple-dp-with-short-code/
const NUM_TAKE: usize = 3;

impl Solution {
    pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
        let n = stone_value.len();
        let mut dp = vec![i32::MIN; n + 1];
        dp[n] = 0;
        for i in (0..n).rev() {
            let mut sum = 0;
            for j in 0..NUM_TAKE.min(n - i) {
                sum += stone_value[i + j];
                dp[i] = dp[i].max(sum - dp[i + j + 1]);
            }
        }
        let mut ret = if dp[0] > 0 { "Alice" } else { "Bob" };
        if dp[0] == 0 { ret = "Tie"; }
        String::from(ret)
    }
}