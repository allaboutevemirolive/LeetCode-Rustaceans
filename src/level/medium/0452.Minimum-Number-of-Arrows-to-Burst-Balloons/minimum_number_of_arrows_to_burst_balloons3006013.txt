// https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/solutions/3006013/rust-solution/
use std::cmp;
impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        let mut a = points;
        a.sort_unstable();
        let mut ans = 1;

        let mut last = a[0][1];
        for i in 1..a.len() {
            if a[i][0] > last {
                ans += 1;
                last = a[i][1];                
            }
            else {
                last = cmp::min(a[i][1], last);
            }
        }
        ans

    }
}