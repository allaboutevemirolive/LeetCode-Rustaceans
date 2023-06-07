// https://leetcode.com/problems/range-module/solutions/2116943/rust-treemap-might-not-be-the-most-idiomatic-rust-solution/
use std::collections::{BTreeMap, LinkedList};


#[derive(Debug)]
struct RangeModule {
    intervals: BTreeMap<i32, i32>,
}

impl RangeModule {
    fn new() -> Self {
        RangeModule {
            intervals: BTreeMap::new(),
        }
    }

    fn add_range(&mut self, mut left: i32, mut right: i32) {
        if let Some((bgn, end)) = self.intervals.range(..=left).next_back() {
            if *end >= left {
                left = std::cmp::min(left, *bgn)
            }
        }

        if let Some((_, end)) = self.intervals.range(..=right).next_back() {
            if left < *end {
                right = std::cmp::max(right, *end)
            }
        }

        let mut new_intervals: BTreeMap<i32, i32> = self
            .intervals
            .clone()
            .into_iter()
            .filter(|(bgn, _)| *bgn < left || *bgn > right)
            .collect();
        new_intervals.insert(left, right);

        self.intervals = new_intervals;
    }

    fn query_range(&self, left: i32, right: i32) -> bool {
        match self.intervals.range(..=left).next_back() {
            Some((_, end)) => *end >= right,
            None => false,
        }
    }

    fn remove_range(&mut self, left: i32, right: i32) {
        if let Some((_, end)) = self.intervals.range(..=right).next_back() {
            if *end > right {
                self.intervals.insert(right, *end);
            }
        }

        if let Some((_, end)) = self.intervals.range_mut(..=left).next_back() {
            if *end > left {
                *end = left;
            }
        }

        let new_intervals: BTreeMap<i32, i32> = self
            .intervals
            .clone()
            .into_iter()
            .filter(|(bgn, _)| *bgn < left || *bgn >= right)
            .collect();

        self.intervals = new_intervals;
    }
}
