// https://leetcode.com/problems/reducing-dishes/solutions/3353435/rust-greedy/
impl Solution {
    pub fn max_satisfaction(mut s: Vec<i32>) -> i32 {

        s.sort_unstable();

        let mut sum = 0; // sum of satisfication you already pick
        let mut ans = 0; // satisfication
        
        // pick from large to small
        for s in s.into_iter().rev() {
            // if you choose to add new one, what you already pick
            // will count one more time, i.e. ans = ans + sum + s
            // if ans get increased, we can take it
            // otherwise, we can stop and never moves on as further
            // ones are even smaller than current pick
            if sum + s >= 0 {
                sum += s;
                ans += sum;
            } else {
                break;
            }
        }
        ans
    }
}