// https://leetcode.com/problems/data-stream-as-disjoint-intervals/solutions/874323/rust-ordered-set-with-interval-newtype/
use std::collections::BTreeSet;
use std::cmp::*;

#[derive(Eq, Clone, Copy)]
struct Interval {
    start: i32,
    end: i32,
}

impl Interval {
    fn num(x: i32) -> Self {
        Self { start: x, end: x }
    }
    
    fn merge(self, other: Self) -> Self {
        Self {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
    
    fn is_inside_of(&self, other: &Self) -> bool {
        self.start >= other.start && self.end <= other.end
    }
}

impl Ord for Interval {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.eq(&other) {
            Ordering::Equal
        } else {
            self.start.cmp(&other.start)
        }
    }
}

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Interval {
    fn eq(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.start
            || self.start <= other.end && self.end >= other.end
            || self.is_inside_of(&other)
            || other.is_inside_of(&self)
    }
}

#[derive(Default)]
struct SummaryRanges {
    set: BTreeSet<Interval>,
}

impl SummaryRanges {
    fn new() -> Self { Self::default() }
    
    fn add_num(&mut self, val: i32) {
        let this = Interval::num(val);
        let x = self.set.take(&Interval::num(val - 1)).unwrap_or(this);
        let y = self.set.take(&Interval::num(val + 1)).unwrap_or(this);
        self.set.insert(Interval::merge(x, y));
    }
    
    fn get_intervals(&self) -> Vec<Vec<i32>> {
        self.set.iter().map(|i| vec![i.start, i.end]).collect()
    }
}