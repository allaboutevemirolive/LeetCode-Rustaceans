// https://leetcode.com/problems/search-a-2d-matrix/solutions/2595310/rust-flatten-binary-search/
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        match matrix.iter()
            .cloned()
            .flatten()
            .collect::<Vec<i32>>()
            .binary_search(&target) {
                Ok(_) => true,
                Err(_) => false,
        }
    }
}