// https://leetcode.com/problems/reducing-dishes/solutions/584912/rust-brut-force-0ms/
impl Solution {
    fn value(slice: &[i32]) -> i32 {
        slice.iter().enumerate().map(|(i, v)| ((i + 1) as i32) * *v).sum()
    }

    pub fn max_satisfaction(mut satisfaction: Vec<i32>) -> i32 {
        satisfaction.sort();
        (0..satisfaction.len()).map(|i| {
            Solution::value(&satisfaction[i..])
        }).fold(std::i32::MIN, |a, b| a.max(b)).max(0)
    }
}