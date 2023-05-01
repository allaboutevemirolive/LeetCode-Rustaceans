// https://leetcode.com/problems/longest-consecutive-sequence/solutions/2240782/rust-with-generic-union-by-size/
use std::hash::Hash;
use std::collections::HashMap;

struct UnionBySize<T> {
    parent: HashMap<T, T>,
    size: HashMap<T, i32>,
}

impl<T> UnionBySize<T>
where T : Copy + Eq + Hash {
    fn new() -> Self {
        Self {
            parent: HashMap::new(),
            size: HashMap::new(),
        }
    }

    fn contains(&self, x: T) -> bool {
        self.parent.contains_key(&x)
    }

    fn make_set(&mut self, x: T) {
        self.parent.insert(x, x);
        self.size.insert(x, 1);
    }

    fn find_set(&mut self, x: T) -> T {
        let mut p = *self.parent.get(&x).unwrap();
        if x != p {
            p = self.find_set(p);
            self.parent.insert(x, p);
        }
        p
    }

    fn union_by_size(&mut self, x: T, y: T) {
        let mut xs = self.find_set(x);
        let mut ys = self.find_set(y);
        if xs != ys {
            if self.size.get(&xs).unwrap() < self.size.get(&ys).unwrap() {
                std::mem::swap(&mut xs, &mut ys);
            }
            self.parent.insert(ys, xs);
            *self.size.get_mut(&xs).unwrap() += self.size.remove(&ys).unwrap();
        }
    }

    fn largest_size(&self) -> i32 {
        *self.size.values().max().unwrap_or(&0)
    }
}

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut u = UnionBySize::new();
        for &x in nums.iter() {
            if !u.contains(x) {
                u.make_set(x);
                if u.contains(x - 1) {
                    u.union_by_size(x - 1, x);
                }
                if u.contains(x + 1) {
                    u.union_by_size(x + 1, x);
                }
            }
        }
        u.largest_size()
    }
}