// https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/solutions/3001950/rust-greedy-functional-style-with-comments/
impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by_key(|b| b[1]);
        points.iter().skip(1).fold((1, points[0][1]), |(rez, right), b|
            if right < b[0] {
                (rez + 1, b[1])
            } else {
                (rez, right)
            }
        ).0
    }
}