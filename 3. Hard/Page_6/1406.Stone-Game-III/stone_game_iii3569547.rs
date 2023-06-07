// https://leetcode.com/problems/stone-game-iii/solutions/3569547/rust-top-down-dp/
impl Solution {
    pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
        fn dfs(i: usize, stones: &Vec<i32>, table: &mut Vec<i32>) -> i32 {
            if i == stones.len() { return 0 }
            if table[i] != i32::MIN { return table[i] }

            let mut score = 0;

            for j in i..(stones.len().min(i + 3)) {
                score += stones[j];
                table[i] = table[i].max(score - dfs(j + 1, stones, table));
            }

            table[i]
        }

        let mut table = vec![i32::MIN; stone_value.len()];
        let score = dfs(0, &stone_value, &mut table);

        match score {
            0 => "Tie".to_string(),
            i32::MIN..=-1 => "Bob".to_string(),
            1..=i32::MAX => "Alice".to_string(),
        }
    }
}