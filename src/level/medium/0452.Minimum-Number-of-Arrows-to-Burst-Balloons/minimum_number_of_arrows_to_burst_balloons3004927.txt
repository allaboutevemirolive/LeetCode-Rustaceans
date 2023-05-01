// https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/solutions/3004927/rust-57ms-beats-95/
impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {

        use std::i32::{MIN, MAX};
        use std::cmp;

        let mut sorted_points = points.clone();
        sorted_points.sort_unstable_by_key(|p| p[0]);

        let mut num_arrows: u32 = 0;
        let mut cur_range = vec![MIN, MAX];
        for balloon in &sorted_points{                 
            if balloon[0] <= cur_range[1]{
                cur_range[0] = balloon[0];
                cur_range[1] = cmp::min(cur_range[1], balloon[1]);
            } else {
                num_arrows += 1;
                cur_range = (*balloon).clone();
            }
        }
        num_arrows += 1;
        return num_arrows as i32;
        
    }
}