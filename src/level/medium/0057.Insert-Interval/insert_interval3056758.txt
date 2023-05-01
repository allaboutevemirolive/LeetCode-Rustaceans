// https://leetcode.com/problems/insert-interval/solutions/3056758/rust-elixir-different-approaches/
impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let left = intervals.partition_point(|v| v[1] < new_interval[0]);
        let right = intervals.partition_point(|v| v[0] <= new_interval[1]);
        if left < intervals.len() {
            new_interval[0] = new_interval[0].min(intervals[left][0]);
        }
        if right > 0 {
            new_interval[1] = new_interval[1].max(intervals[right - 1][1]);
        }
        intervals.splice(left..right, [new_interval]);
        intervals
    }
}