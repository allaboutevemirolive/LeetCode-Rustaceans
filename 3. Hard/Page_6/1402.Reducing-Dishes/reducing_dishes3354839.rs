// https://leetcode.com/problems/reducing-dishes/solutions/3354839/rust-one-loop/
impl Solution {
    pub fn max_satisfaction(satisfaction: Vec<i32>) -> i32 {
        let mut sorted = satisfaction.clone();
        sorted.sort_by(|a, b| b.cmp(a));
        let mut cumsum = 0;
        let mut partsum = 0;
        let mut result = 0;
        for n in sorted {
            partsum += n;
            cumsum += partsum;
            if cumsum > result {
                result = cumsum;
            }
        }

        return result;
    }
}