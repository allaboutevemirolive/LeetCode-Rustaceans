// https://leetcode.com/problems/triangle/solutions/1169793/rust-solution/
impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        (0..triangle.len() - 1)
            .rev()
            .fold(triangle[triangle.len() - 1].clone(), |mut acc, i| {
                (0..=i).for_each(|j| acc[j] = triangle[i][j] + acc[j].min(acc[j + 1]));
                acc
            })[0]
    }
}