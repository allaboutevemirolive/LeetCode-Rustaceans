// https://leetcode.com/problems/reducing-dishes/solutions/3353848/rust-greedy-solution/
impl Solution {
    pub fn max_satisfaction(satisfaction: Vec<i32>) -> i32 {
        let mut satisfaction = satisfaction;
        let mut res = 0;
        let mut pre_sum = 0;

        satisfaction.sort_by(|a, b| b.cmp(a));

        for s in satisfaction.into_iter() {
            if pre_sum + s >= 0 {
                pre_sum += s;
                res += pre_sum;
            }
        }

        res
    }
}