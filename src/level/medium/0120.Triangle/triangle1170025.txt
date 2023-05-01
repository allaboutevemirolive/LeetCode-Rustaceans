// https://leetcode.com/problems/triangle/solutions/1170025/rust-short-o-1-space/
impl Solution {
    pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
        for y in (0..triangle.len() - 1).rev() {
            for x in (0..triangle[y].len()) {
                triangle[y][x] += triangle[y + 1][x].min(triangle[y + 1][x + 1]);
            }
        }
        triangle[0][0]
    }
}