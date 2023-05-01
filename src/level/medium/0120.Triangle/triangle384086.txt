// https://leetcode.com/problems/triangle/solutions/384086/concise-rust-0ms-o-1-extra-space/
impl Solution {
    pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
        let rows = triangle.len();
        (0..rows-1).rev().for_each(|r| (0..r+1).for_each(|c| 
               triangle[r][c] += std::cmp::min(triangle[r+1][c], triangle[r+1][c+1])));
        triangle[0][0]
    }
}