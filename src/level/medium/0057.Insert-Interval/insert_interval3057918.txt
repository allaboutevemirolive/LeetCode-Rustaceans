// https://leetcode.com/problems/insert-interval/solutions/3057918/rust-binary-search-solution/
impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let ps = intervals.partition_point(|v| v[1] < new_interval[0]);
        let pe = intervals.partition_point(|v| v[0] <= new_interval[1]);
        if ps == pe {
            intervals.insert(ps, new_interval);
        } else {
            let new_interval = vec![
                intervals[ps][0].min(new_interval[0]),
                intervals[pe - 1][1].max(new_interval[1]),
            ];
            intervals.drain(ps..pe);
            intervals.insert(ps, new_interval);
        }
        intervals
    }
}