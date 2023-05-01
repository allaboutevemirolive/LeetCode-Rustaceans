// https://leetcode.com/problems/maximal-square/solutions/333108/rust-solution-8ms-beats-100/
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() { return 0 }
        
        let mut max_area = 0;
        
        for (row_idx, row) in matrix.iter().enumerate() {
            for (col_idx, col) in row.iter().enumerate() {
                if col == &'0' { continue }
                let area = Self::calc_area(row_idx, col_idx, &matrix);
                max_area = std::cmp::max(area, max_area);
            }
        }
        
        max_area
    }
    
    fn calc_area(row_idx: usize, col_idx: usize, matrix: &Vec<Vec<char>>) -> i32 {
        let num_of_rows = matrix.len();
        let num_of_cols = matrix[0].len();
        
        let mut area = 1;
        
        let mut next_col = col_idx;
        let mut next_row = row_idx;
        
        'main: loop {
            let mut i = row_idx;
            while i <= next_row {
                if matrix[i][next_col] == '0' { break 'main }
                i += 1;
            }
            let mut j = col_idx;
            while j <= next_col {
                if matrix[next_row][j] == '0' { break 'main }
                j += 1;
            }
            
            area = ((next_col - col_idx + 1) * (next_col - col_idx + 1)) as i32;
            
            if next_col == num_of_cols - 1 || next_row == num_of_rows - 1 { break 'main }
            
            next_col += 1;
            next_row += 1;
        }
        
        
        area
    }
}