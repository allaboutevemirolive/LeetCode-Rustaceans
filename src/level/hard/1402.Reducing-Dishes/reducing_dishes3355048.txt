// https://leetcode.com/problems/reducing-dishes/solutions/3355048/rust-solution/
impl Solution {
    pub fn max_satisfaction(mut satisfaction: Vec<i32>) -> i32 {
        satisfaction.sort();
        let mut d = 0;
        let mut result = 0;
        while let Some(x) = satisfaction.pop() {
            d += x;
            if d <= 0 {
                break;
            }
            result += d;
        }
        result
    }
}