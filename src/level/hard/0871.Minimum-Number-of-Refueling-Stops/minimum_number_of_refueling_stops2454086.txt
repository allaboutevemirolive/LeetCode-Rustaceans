// https://leetcode.com/problems/minimum-number-of-refueling-stops/solutions/2454086/rust-priority-queue-soliton/
use std::collections::BinaryHeap;

impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        let mut pq = BinaryHeap::new();
        let mut cur_fuel = start_fuel;
        let mut i = 0;
        let mut stops = 0;
        while cur_fuel < target {
            while i < stations.len() && cur_fuel >= stations[i][0] {
                pq.push(stations[i][1]);
                i += 1;
            }
            if let Some(fuel) = pq.pop() {
                cur_fuel += fuel
            } else {
                return -1;
            }
            stops += 1;
        }
        stops
    }
}