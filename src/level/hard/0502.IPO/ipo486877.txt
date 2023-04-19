// https://leetcode.com/problems/ipo/solutions/486877/rust-copy-for-learn-using-binaryheap/
use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
    let mut w = w;
    let mut x: Vec<(i32, i32)> = Vec::new();
    for i in 0..profits.len() {
        x.push((profits[i], capital[i]));
    }
    x.sort_by(|a, b| { a.1.cmp(&b.1) });

    let mut heap = BinaryHeap::new();
    let mut j = 0;
    for i in 0..k {
        while j < profits.len() && x[j as usize].1 <= w {
            heap.push(x[j as usize].0);
            j += 1;
        }
        if !heap.is_empty() {
            w += heap.pop().unwrap();
        }
    }
    w
}
