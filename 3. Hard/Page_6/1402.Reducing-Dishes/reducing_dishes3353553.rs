// https://leetcode.com/problems/reducing-dishes/solutions/3353553/rust-functional-style-o-nlog-n-o-1-solution/
impl Solution {
    pub fn max_satisfaction(mut satisfaction: Vec<i32>) -> i32 {
        satisfaction.sort_unstable_by_key(|el| -el);
        satisfaction.into_iter().scan((0, 0), |&mut (ref mut sum, ref mut total), sat| {
            *sum += sat;
            *total += *sum;
            if *sum > 0 { Some(*total) } else { None }
        }).last().unwrap_or(0)
    }
}