// https://leetcode.com/problems/unique-paths-ii/solutions/1107661/rust-0ms-solution/
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let mut field: Vec<Vec<Option<i32>>> = obstacle_grid.into_iter().map(
            |r| r.into_iter().map(|is_rock| {
                match is_rock {
                    0 => Some(0),
                    _ => None,
                }
            }).collect::<Vec<_>>()
        ).collect();
        
        
        field[0][0] = field[0][0].map(|_| 1);
        
        for row in 0..field.len() {
            for col in 0..field[row].len() {
                field[row][col] = field[row][col].map(|v| {
                    let mut total = v;
                    if col > 0 {
                        total += field[row][col - 1].unwrap_or(0);
                    }
                    if row > 0 {
                        total += field[row - 1][col].unwrap_or(0);
                    }
                    total
                })
            }
        }
        
        field[field.len() - 1][field[0].len() - 1].unwrap_or(0)
    }
}