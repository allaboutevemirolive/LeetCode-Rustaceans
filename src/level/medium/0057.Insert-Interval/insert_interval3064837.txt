// https://leetcode.com/problems/insert-interval/solutions/3064837/rust-sorting-solution/
impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        intervals.push(new_interval);
        intervals.sort();

        let (mut res, mut curr) = (vec![], (intervals[0][0], intervals[0][1]));

        for interval in &intervals[1..] {
            let (s1, e1) = curr;
            let (s2, e2) = (interval[0], interval[1]);

            if e1 >= s2 {
                curr.1 = e1.max(e2)
            } else {
                res.push(vec![s1, e1]);
                curr = (s2, e2)
            }
        }

        res.push(vec![curr.0, curr.1]);
        res
    }
}