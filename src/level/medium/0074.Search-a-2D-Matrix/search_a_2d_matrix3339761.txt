// https://leetcode.com/problems/search-a-2d-matrix/solutions/3339761/rust-solution/
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        fn search_row(matrix: &Vec<Vec<i32>>, target: i32) -> usize {
            let mut lo = 0;
            let mut hi = matrix.len() - 1;
            while lo < hi {
                let mid = lo + (hi - lo) / 2;
                if matrix[mid][matrix[0].len() - 1] >= target {
                    hi = mid;
                } else {
                    lo = mid + 1;
                }
            }
            lo
        }
        fn search_col(matrix: &[i32], target: i32) -> i32 {
            let mut lo = 0i32;
            let mut hi = matrix.len() as i32 - 1;
            while lo <= hi {
                let mid = lo + (hi - lo) / 2;
                if matrix[mid as usize] == target {
                    return mid as i32;
                } else if matrix[mid as usize] < target {
                    lo = mid + 1;
                } else {
                    hi = mid - 1;
                }
            }
            -1
        }
        let row_idx = search_row(&matrix, target);
        let col_idx = search_col(&matrix[row_idx], target);
        col_idx >= 0        
    }
}