// https://leetcode.com/problems/surrounded-regions/solutions/1439352/rust/
impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        
        fn mark(ch : char, board: &mut Vec<Vec<char>>, x : i32, y : i32, rows : i32, cols : i32, directions : &Vec<Vec<i32>>) {
            board[x as usize][y as usize] = ch;
            for direction in directions.iter() {
                let new_x = x + direction[0];
                let new_y = y + direction[1];
                
                if new_x >= 0 && new_y >= 0 && new_x < rows && new_y < cols && board[new_x as usize][new_y as usize] == 'O' {
                    mark(ch, board, new_x, new_y, rows, cols, directions);
                }
            }

        }

        let directions = vec![vec![-1, 0], vec![1, 0], vec![0, 1], vec![0, -1]];
        let rows = board.len() as i32;
        let cols = board[0].len() as i32;

        for i in 0 .. rows {
            for j in 0 .. cols {
                if (i == 0 || j == 0 || i == rows -1 || j == cols - 1) && board[i as usize][j as usize] == 'O' {
                  mark('$', board, i, j , rows, cols, &directions);
                }
            }
        }

        for i in 0 .. rows {
            for j in 0 .. cols {
                let current = board[i as usize][j as usize];

                if current == 'O' {
                    board[i as usize][j as usize] = 'X';
                } else if current == '$' {
                    board[i as usize][j as usize] = 'O';
                }
            }
        }
    }
}
