// https://leetcode.com/problems/merge-intervals/solutions/1837450/rust/
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::cmp::Ordering::*;
        intervals.sort_by(|a, b| match a[0].cmp(&b[0]) {
            Equal => a[1].cmp(&b[1]),
            Less => Less,
            Greater => Greater,
        });

        let mut res = vec![];

        let mut i = 0;
        while i < intervals.len() {
            let left = intervals[i][0];
            let mut right = intervals[i][1];
            let mut j = i + 1;
            while j < intervals.len() {
                if intervals[j][0] > right {
                    // no overlap
                    break;
                }

                // has overlap
				if intervals[j][1] > right { // totally overlap
                	right = intervals[j][1];
				}
                j += 1;
            }
            res.push(vec![left, right]);
            i = j;
        }

        res
    }
}