// https://leetcode.com/problems/game-of-life/solutions/523238/rust-solution-not-accepted/
impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) -> Vec<Vec<i32>>{
        // m rows, n columns
        let m = board.len();
        let n = board[0].len();
        
        let valid_point = |i: i32, j: i32| -> bool{
            0 <= i && i < m as i32 && 0 <= j && j < n as i32
        };
        
        let live_neighbours = |i: usize, j: usize| -> i32{
            let neigb_deltas = [
                (1, 1), (1, 0), (1, -1), 
                (0, 1), (0, -1),
                (-1, 1), (-1, 0), (-1, -1)
            ];
            let mut result = 0;
            for (a, b) in neigb_deltas.iter(){
                let x = a + i as i32;
                let y = b + j as i32;
                if valid_point(x, y){
                    result += board[x as usize][y as usize];
                }
            }
            result
        };
        
        let next_state = |i: usize, j: usize| -> i32{
            let ln = live_neighbours(i,j);
            if ln < 2 || ln > 3 {
                0
            } else if board[i][j] == 0 && ln == 2 {
                0
            } else {
                1
            }
        };
        
        let mut result = Vec::<Vec<i32>>::with_capacity(m);
        for i in 0..m{
            result.push(vec![0; n]);
        }
        
        for i in 0..m{    
            for j in 0..n{
                result[i][j] = next_state(i, j);
            }
        }
        
        println!("input:");
        for i in 0..m{    
            println!("{:?}", board[i])
        }
        
        println!("output:");
        for i in 0..m{    
            println!("{:?}", result[i])
        }
        
        result
    }
}