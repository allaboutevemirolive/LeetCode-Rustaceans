// https://leetcode.com/problems/range-module/solutions/847605/rust-72ms-btreeset/
use std::cmp::Ordering;
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Interval {
    left: i32,
    right: i32,
}

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Interval {
    fn cmp(&self, other: &Self) -> Ordering {
        self.right
            .cmp(&other.right)
            .then(self.left.cmp(&other.left))
    }
}

#[derive(Debug, Default)]
struct RangeModule {
    ranges: BTreeSet<Interval>,
}

impl RangeModule {
    fn new() -> Self {
        Default::default()
    }

    fn add_range(&mut self, mut left: i32, mut right: i32) {
        while let Some(&iv) = self
            .ranges
            .range(
                Interval {
                    left: 0,
                    right: left - 1,
                }..,
            )
            .next()
        {
            if right < iv.left {
                break;
            }
            left = std::cmp::min(left, iv.left);
            right = std::cmp::max(right, iv.right);
            self.ranges.remove(&iv);
        }
        self.ranges.insert(Interval { left, right });
    }

    fn query_range(&self, left: i32, right: i32) -> bool {
        if let Some(&iv) = self
            .ranges
            .range(
                Interval {
                    left: 0,
                    right: left,
                }..,
            )
            .next()
        {
            iv.left <= left && right <= iv.right
        } else {
            false
        }
    }

    fn remove_range(&mut self, left: i32, right: i32) {
        let mut todo = vec![];
        while let Some(&iv) = self
            .ranges
            .range(
                Interval {
                    left: 0,
                    right: left,
                }..,
            )
            .next()
        {
            if right < iv.left {
                break;
            }
            if iv.left < left {
                todo.push(Interval {
                    left: iv.left,
                    right: left,
                });
            }
            if right < iv.right {
                todo.push(Interval {
                    left: right,
                    right: iv.right,
                });
            }
            self.ranges.remove(&iv);
        }
        for it in todo {
            self.ranges.insert(it);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_range() {
        let mut range = RangeModule::new();
        range.add_range(10, 20);
        range.remove_range(14, 16);
        println!("{:?}", range);
        assert!(range.query_range(10, 14));
        assert!(!range.query_range(13, 15));
        assert!(range.query_range(16, 17));
    }

    #[test]
    fn test_add_range_02() {
        let mut range = RangeModule::new();
        range.add_range(10, 180);
        println!("{:?}", range);
        range.add_range(150, 200);
        range.add_range(250, 500);
        println!("{:?}", range);
        assert!(range.query_range(50, 100));
        assert!(!range.query_range(180, 300));
        assert!(!range.query_range(600, 1000));

        range.remove_range(50, 150);
        println!("{:?}", range);
        assert!(!range.query_range(50, 100));
    }
}