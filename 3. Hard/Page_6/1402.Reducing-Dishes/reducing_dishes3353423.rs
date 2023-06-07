// https://leetcode.com/problems/reducing-dishes/solutions/3353423/rust-dp-solution/
impl Solution {
    pub fn max_satisfaction(mut satisfaction: Vec<i32>) -> i32 {
        let (mut cur, mut res) = (0, 0);
        satisfaction.sort();
        while !satisfaction.is_empty() && satisfaction.last().unwrap() + cur > 0 {
            cur += satisfaction.pop().unwrap();
            res += cur;
        }
        res
    }
}
