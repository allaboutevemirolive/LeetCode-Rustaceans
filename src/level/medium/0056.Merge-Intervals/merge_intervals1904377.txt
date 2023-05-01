// https://leetcode.com/problems/merge-intervals/solutions/1904377/rust/
fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
	intervals.sort_by_key(|interval| interval[0]);

	let mut output = Vec::with_capacity(intervals.len());
	let mut iter = intervals.into_iter();
	let mut current = iter.next().unwrap();
	for interval in iter {
		if interval[0] <= current[1] {
			current[1] = current[1].max(interval[1]);
		} else {
			output.push(current);
			current = interval;
		}
	}
	output.push(current);
	output
}