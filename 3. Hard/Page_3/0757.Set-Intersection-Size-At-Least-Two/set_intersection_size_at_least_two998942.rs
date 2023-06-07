// https://leetcode.com/problems/set-intersection-size-at-least-two/solutions/998942/rust-o-nlogn-solution/
// We assume `start != end`.
#[derive(Debug, Copy, Clone)]
struct Interval {
    start: i32,
    end: i32,
}

 fn intersection_size_two(mut intervals: Vec<Vec<i32>>) -> i32 {
    intervals.sort();
    let intervals: Vec<Interval> = intervals
        .into_iter()
        .map(|xs| Interval {
            start: xs[0],
            end: xs[1],
        })
        .collect();
    let mut num_satisfied = intervals.iter().map(|_| 0).collect::<Vec<i32>>();
    let mut set_size = 0;
    let mut i = 0;

    while i < intervals.len() {
        assert!(num_satisfied[i] <= 2);
        if num_satisfied[i] == 2 {
            i += 1;
            continue;
        }

        let mut overlapped = false;
        let mut j = i + 1;
        while j < intervals.len() && intervals[j].start <= intervals[i].end {
            // The interval at `j` start and ends within the interval at `i`.
            // Therefore, if we satisfy the interval at `j`, we satisfy all intervals in `[i, j]`.
            if intervals[j].end <= intervals[i].end {
                i = j;
            }
            j += 1;
        }

        let to_satisfy = 2 - num_satisfied[i];
        set_size += to_satisfy;
        for k in i..j {
            let k_satisfied = std::cmp::min(to_satisfy, intervals[i].end - intervals[k].start + 1);
            num_satisfied[k] += k_satisfied;
        }

        i += 1;
    }

    set_size
}

impl Solution {
    pub fn intersection_size_two(mut intervals: Vec<Vec<i32>>) -> i32 {
        intersection_size_two(intervals)
    }
}