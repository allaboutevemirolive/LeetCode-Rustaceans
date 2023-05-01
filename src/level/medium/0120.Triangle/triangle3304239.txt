// https://leetcode.com/problems/triangle/solutions/3304239/rust-2-solutions/
impl Solution {
    pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
        fn top_down(mut triangle: Vec<Vec<i32>>) -> i32 {
            let mut ans = i32::MAX;
            for i in 1..triangle.len() {
                for j in 0..triangle[i].len() {
                    let left = if j > 0 { j - 1 } else { j };
                    let right = if j < triangle[i - 1].len() { j } else { j - 1 };
                    triangle[i][j] += triangle[i - 1][right].min(triangle[i - 1][left])
                }
            }
            for i in 0..triangle[triangle.len() - 1].len() {
                ans = ans.min(triangle[triangle.len() - 1][i]);
            }
            ans
        }
        fn bottom_up(mut triangle: Vec<Vec<i32>>) -> i32 {
            // Traverse the triangle from bottom to top
            // The minimum path sum is stored in the first element of the dp array
            // For each element in the current row, update the dp array
            let h = triangle.len();
            for i in (0..h - 1).rev() {
                for j in 0..triangle[i].len() {
                    triangle[i][j] += triangle[i + 1][j + 1].min(triangle[i + 1][j]);
                }
            }
            triangle[0][0]
        }
        bottom_up(triangle)    
    }
}