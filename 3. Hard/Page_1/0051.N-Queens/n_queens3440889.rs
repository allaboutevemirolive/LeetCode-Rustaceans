// https://leetcode.com/problems/n-queens/solutions/3440889/rust-solution/
impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        use std::collections::HashSet;
        fn backtrack(
            board: &mut Vec<Vec<String>>,
            row: i32,
            cols: &mut HashSet<i32>,
            diag: &mut HashSet<i32>,
            anti_diag: &mut HashSet<i32>,
            ans: &mut Vec<Vec<String>>,
        ) {
            if row == board.len() as i32 {
                let board = board
                    .clone()
                    .into_iter()
                    .map(|x| x.into_iter().collect::<String>())
                    .collect::<Vec<_>>();
                ans.push(board);
                return;
            }
            for col in 0..board[0].len() as i32 {
                let i = row - col;
                let j = row + col;
                if cols.contains(&col) || diag.contains(&i) || anti_diag.contains(&j) {
                    continue;
                }
                cols.insert(col);
                diag.insert(i);
                anti_diag.insert(j);
                board[row as usize][col as usize] = "Q".to_string();
                backtrack(board, row + 1, cols, diag, anti_diag, ans);
                board[row as usize][col as usize] = ".".to_string();
                cols.remove(&col);
                diag.remove(&i);
                anti_diag.remove(&j);
            }
        }
        let mut board = vec![vec![".".to_string(); n as usize]; n as usize];
        let mut ans = vec![];
        backtrack(
            &mut board,
            0,
            &mut HashSet::new(),
            &mut HashSet::new(),
            &mut HashSet::new(),
            &mut ans,
        );
        ans        
    }
}