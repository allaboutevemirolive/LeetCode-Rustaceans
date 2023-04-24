// https://leetcode.com/problems/reducing-dishes/solutions/1127017/rust-simple-solution/
impl Solution {
    pub fn max_satisfaction(mut satisfaction: Vec<i32>) -> i32 {
        satisfaction.sort();
        let mut score = 0;
        for i in 0..satisfaction.len() {
            let mut temp_score = 0;
            let mut multiplier = 1;
            for j in 0..satisfaction.len() {
                temp_score+=satisfaction[j]*multiplier;
                multiplier+=1;
            }
            if temp_score>score{
                score=temp_score;
            }
            satisfaction.remove(0);
        }
        
        score
    }
}