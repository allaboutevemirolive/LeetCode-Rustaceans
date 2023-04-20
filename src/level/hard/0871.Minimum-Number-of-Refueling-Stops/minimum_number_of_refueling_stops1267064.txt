// https://leetcode.com/problems/minimum-number-of-refueling-stops/solutions/1267064/rust-max-heap-greedy-solution/
use std::collections::BinaryHeap;

impl Solution {
    pub fn min_refuel_stops(target: i32, mut fuel: i32, mut stations: Vec<Vec<i32>>) -> i32 {
        stations.push(vec![target, 0]);
        let mut heap : BinaryHeap<i32> = BinaryHeap::new(); // unused stations seen so far
        let mut k = 0; // number of refuels so far
        for s in stations.iter() {
            while fuel < s[0] { // can't reach s
                match heap.pop() {
                    Some(f) => {  // we use the largest station to refuel
                        fuel = fuel + f; 
                        k = k + 1
                    },
                    None => return -1  // no usable station: can't reach target
                }
            }
            heap.push(s[1]);
        }
        k
    }
}