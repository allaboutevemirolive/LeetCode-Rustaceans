// https://leetcode.com/problems/search-a-2d-matrix/solutions/2575610/rust-binary-search-by/
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        use std::cmp::Ordering;

        let row = matrix.binary_search_by(|probe| {
            if *probe.first().unwrap() > target {
                Ordering::Greater
            } else if *probe.last().unwrap() < target {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });
        if let Ok(idx) = row {
            matrix[idx].binary_search(&target).is_ok()
        } else {
            false
        }
    }
}