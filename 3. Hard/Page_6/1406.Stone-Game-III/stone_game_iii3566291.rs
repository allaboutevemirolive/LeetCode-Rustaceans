// https://leetcode.com/problems/stone-game-iii/solutions/3566291/rust-dp/
impl Solution {
    pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
        let mut dp = vec![i32::MIN; stone_value.len()];
        Self::foo(&stone_value, &mut dp, 0);

        String::from(match dp[0] {
            0 => "Tie",
            1.. => "Alice",
            _ => "Bob",
        })
    }

    fn foo(stone_value: &[i32], dp: &mut [i32], i: usize) -> i32 {
        if i >= stone_value.len() {
            return 0;
        }

        if dp[i] != i32::MIN {
            return dp[i];
        }

        let mut t = i32::MIN;
        let mut sum = 0;
        for x in 0..3 {
            if i + x >= stone_value.len() {
                break;
            }

            sum += stone_value[i + x];
            t = t.max(sum - Self::foo(stone_value, dp, i + x + 1));
        }
        dp[i] = t;
        t
    }
}