// https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/solutions/3002044/rust-solution-using-sort/
use std::cmp::min;

impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_by_key(|x| x[0]);
        let mut prev: Option<Vec<i32>> = None;
        let mut arrow = 1;

        for point in points {
            if let Some(p) = prev.clone() {
                if point[0] <= p[1] {
                    prev = Some(vec![point[0], min(p[1], point[1])]);
                } else {
                    arrow += 1;
                    prev = Some(point);
                }
            } else {
                prev = Some(point);
            }
        }

        arrow
    }
}