// https://leetcode.com/problems/search-a-2d-matrix/solutions/2375837/rust-binary-search/
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let rows = matrix.len();
        let cols = matrix[0].len();
        
        
        let mut low = 0;
        let mut high = rows*cols;
        
        
        while low < high {
            let mid = low + ((high - low) / 2);
            
            let row = mid / cols;
            let col = mid % cols;
            
            if matrix[row][col] == target {
                return true;
            }
            
            if matrix[row][col] > target {
                high = mid;
            } else {
                low = mid + 1;
            }   
        }
        false
    }
}