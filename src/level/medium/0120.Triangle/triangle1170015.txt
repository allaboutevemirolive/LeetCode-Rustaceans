// https://leetcode.com/problems/triangle/solutions/1170015/rust-bottom-up-dp-time-o-n-space-o-1/
impl Solution {
    pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
        for i in 1..triangle.len() {
            for j in 0..triangle[i].len() {
                if j == 0 {
                    triangle[i][j] = triangle[i][j] + triangle[i-1][j];
                } else if j == triangle[i].len() - 1 {
                    triangle[i][j] = triangle[i][j] + triangle[i-1][j-1];
                } else {
                    triangle[i][j] = triangle[i][j] + triangle[i-1][j-1].min(triangle[i-1][j]);
                }
            }
        }
        
        *triangle[triangle.len()-1].iter().min().unwrap()
    }
}