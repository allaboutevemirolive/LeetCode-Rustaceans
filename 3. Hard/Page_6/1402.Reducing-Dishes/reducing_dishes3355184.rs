// https://leetcode.com/problems/reducing-dishes/solutions/3355184/rust-greedy-solution/
impl Solution {
    pub fn max_satisfaction(mut satisfaction: Vec<i32>) -> i32 {
        satisfaction.sort();
        let mut i = (satisfaction.len()-1) as i32;
        let mut increment = 0;
        let mut max_score = 0;
        while i>=0 && increment+satisfaction[i as usize]>0  {
            increment+=satisfaction[i as usize];
            max_score+=increment;
            i-=1;
        }
        max_score
    }
}