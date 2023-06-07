// https://leetcode.com/problems/minimum-number-of-refueling-stops/solutions/828163/rust-translated-4ms-100/
impl Solution {
    pub fn min_refuel_stops(target: i32, mut start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        use std::collections::BinaryHeap;

        let mut heap = BinaryHeap::<i32>::new();
        let mut i = 0;
        let mut ans = 0;
        while start_fuel < target {
            while i < stations.len() && stations[i][0] <= start_fuel {
                heap.push(stations[i][1]);
                i += 1;
            }
            if heap.is_empty() {
                return -1;
            }
            start_fuel += heap.pop().unwrap();
            ans += 1;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_refuel_stops() {
        assert_eq!(Solution::min_refuel_stops(1, 1, vec![]), 0)
    }

    #[test]
    fn test_min_refuel_stops_02() {
        assert_eq!(Solution::min_refuel_stops(100, 1, vec![vec![10, 100]]), -1)
    }

    #[test]
    fn test_min_refuel_stops_03() {
        assert_eq!(
            Solution::min_refuel_stops(
                100,
                10,
                vec![vec![10, 60], vec![20, 30], vec![30, 30], vec![60, 40]]
            ),
            2
        )
    }
}