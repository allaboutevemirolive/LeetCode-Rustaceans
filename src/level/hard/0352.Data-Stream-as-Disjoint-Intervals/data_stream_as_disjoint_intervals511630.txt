// https://leetcode.com/problems/data-stream-as-disjoint-intervals/solutions/511630/elegant-rust-use-btreemap/
use std::collections::BTreeMap;

struct SummaryRanges {
    hl: BTreeMap<i32, i32>, //store segment keyed by left point
    hr: BTreeMap<i32, i32>, //store segment keyed by right point
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SummaryRanges {

    /** Initialize your data structure here. */
    fn new() -> Self {
        SummaryRanges{ hl: BTreeMap::new(), hr: BTreeMap::new() }
    }

    fn remove(&mut self, l: i32, r: i32) {
        self.hl.remove(&l);
        self.hr.remove(&r);
    }

    fn insert(&mut self, l: i32, r: i32) {
        self.hl.insert(l, r);
        self.hr.insert(r, l);
    }

    fn add_num(&mut self, val: i32) {
        match (self.hr.get_key_value(&(val - 1)), self.hl.get_key_value(&(val + 1))) {
            (Some((&rp, &lp)), Some((&lq, &rq))) => {
                self.remove(lp, rp);
                self.remove(lq, rq);
                self.insert(lp, rq);
            },
            (Some((&rp, &lp)), None) => {
                self.remove(lp, rp);
                self.insert(lp, rp + 1);
            },
            (None, Some((&lq, &rq))) => {
                self.remove(lq, rq);
                self.insert(lq - 1, rq);
            },
            (None, None) => {
                if self.hr.range(val..).next().filter(|(&_, &l)| l <= val).is_none() {
                    self.insert(val, val);
                }
            }
        }
    }

    fn get_intervals(&self) -> Vec<Vec<i32>> {
        self.hl.iter().map(|(&l, &r)| vec![l, r]).collect()
    }
}