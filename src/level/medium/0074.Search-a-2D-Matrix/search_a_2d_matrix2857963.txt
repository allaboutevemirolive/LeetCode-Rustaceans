// https://leetcode.com/problems/search-a-2d-matrix/solutions/2857963/rust-0ms/
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        fn binary_search(v: &Vec<i32>, target: i32) -> bool {
            let (mut lo, mut hi) = (0,v.len());
            while lo < hi {
                let mid = lo + (hi - lo) / 2;
                if target == v[mid] { return true; }
                if target < v[mid] {
                    hi = mid;
                } else {
                    lo = mid + 1;
                }
            }
            false
        }
        
        // find row
        for row in &matrix {
            if row[row.len() - 1] >= target {
                return binary_search(row, target);
            }
        }
        
        false
    }
}