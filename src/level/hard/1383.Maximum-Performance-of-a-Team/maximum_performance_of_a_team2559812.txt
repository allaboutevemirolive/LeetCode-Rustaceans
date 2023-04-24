// https://leetcode.com/problems/maximum-performance-of-a-team/solutions/2559812/rust-sort-and-heap/
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq)]
struct State {
    efficiency: i32,
    speed: i32,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return other
            .efficiency
            .cmp(&self.efficiency)
            .then_with(|| other.speed.cmp(&self.speed));
    }
}

impl Solution {
    pub fn max_performance(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
        let mut pairs: Vec<State> = efficiency
            .iter()
            .zip(speed)
            .map(|(a, b)| State {
                efficiency: *a,
                speed: b,
            })
            .collect();
        pairs.sort();

        let mut heap: BinaryHeap<i64> = BinaryHeap::new();

        let mut min_efficiency = i64::MIN;
        let mut max_perfomance: i64 = 0;
        let mut max_sum_speed = 0;

        for pair in pairs.iter() {
            let speed = pair.speed as i64;
            let cur_efficiency = pair.efficiency as i64;

            if min_efficiency == i64::MIN {
                min_efficiency = cur_efficiency;
            }

            if heap.len() < k as usize {
                max_sum_speed += speed;
                let sum = i64::min(cur_efficiency, min_efficiency) * (max_sum_speed);
                heap.push(-speed);

                if sum > max_perfomance {
                    min_efficiency = i64::min(cur_efficiency, min_efficiency);
                    max_perfomance = sum;
                }
            } else {
                if let Some(min_team_speed) = heap.peek() {
                    let min_team_speed = -min_team_speed;
                    if max_sum_speed - min_team_speed + speed > max_sum_speed {
                        heap.push(-speed);
                        heap.pop();
                        max_sum_speed = max_sum_speed - min_team_speed + speed;
                    }

                    let sum = i64::min(cur_efficiency, min_efficiency)
                        * (max_sum_speed);
                    if sum > max_perfomance {
                        min_efficiency = i64::min(cur_efficiency, min_efficiency);
                        max_perfomance = sum;
                    }
                }
            }
        }

        (max_perfomance % ((1e9 as i64) + 7)) as i32
    }
}