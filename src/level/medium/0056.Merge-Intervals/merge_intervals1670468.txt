// https://leetcode.com/problems/merge-intervals/solutions/1670468/rust/
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
		fn merge_interval(interval_a: &Vec<i32>, interval_b: &Vec<i32>) -> Option<Vec<i32>> {
			if interval_a[1] >= interval_b[0] {
				Some(vec![interval_a[0], interval_a[1].max(interval_b[1])])
			} else {
				None
			}
		}

		intervals.sort_unstable_by(|x, y| x[0].cmp(&y[0]));

		let mut result = Vec::new();
		let mut merge_candidate = intervals[0].clone();
		for interval in intervals.iter().skip(1) {
			merge_candidate = match merge_interval(&merge_candidate, &interval) {
				None => {
					result.push(merge_candidate);
					interval.clone()
				}
				Some(merged) => {
					merged
				}
			}
		}
		result.push(merge_candidate);

		result
    }