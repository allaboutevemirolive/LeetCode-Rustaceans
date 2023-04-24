// https://leetcode.com/problems/reducing-dishes/solutions/1158159/rust-1402-reducing-dishes/
impl Solution {
    pub fn max_satisfaction(mut satisfaction: Vec<i32>) -> i32 {
        satisfaction.sort();

        let (mut max, mut sum) = (0, 0);

        for i in satisfaction.iter().rev() {
            sum += i;
            if sum > 0 {
                max += sum;
            } else {
                break;
            }
        }
        return max;
    }
}