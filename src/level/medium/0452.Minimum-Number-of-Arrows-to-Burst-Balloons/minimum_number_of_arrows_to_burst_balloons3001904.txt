// https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/solutions/3001904/rust-simple-sort-and-fold/
impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points;
        points.sort();

        let mut curr = points[0][1];
        let mut cnt = 1;
        for i in 1..points.len() {
            if curr >= points[i][0] {
                curr = std::cmp::min(curr, points[i][1]);
            } else {
                curr = points[i][1];
                cnt += 1;
            }
        }
        cnt
    }
}