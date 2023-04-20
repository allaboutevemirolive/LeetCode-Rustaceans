// https://leetcode.com/problems/maximum-frequency-stack/solutions/1086967/rust-2-solutions-short-binaryheap-hashmap-based-solutions/
use std::collections::{BinaryHeap, HashMap};

#[derive(Default)]
struct FreqStack {
    max_heap: BinaryHeap<(usize, usize, i32)>,
    freqs: HashMap<i32, usize>,
    push_count: usize,
}

impl FreqStack {
    fn new() -> Self {
        Default::default()
    }

    fn push(&mut self, x: i32) {
        *self.freqs.entry(x).or_default() += 1;
        self.max_heap.push((self.freqs[&x], self.push_count, x));
        self.push_count += 1;
    }

    fn pop(&mut self) -> i32 {
        let x = self.max_heap.pop().unwrap().2;
        *self.freqs.get_mut(&x).unwrap() -= 1;
        x
    }
}