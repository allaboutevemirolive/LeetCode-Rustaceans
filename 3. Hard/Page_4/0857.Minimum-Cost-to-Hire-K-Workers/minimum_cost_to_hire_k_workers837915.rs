// https://leetcode.com/problems/minimum-cost-to-hire-k-workers/solutions/837915/rust-translated-8ms-100/
use std::cmp::Ordering;
const EPSILON: f64 = 1.0e-5;

#[derive(Debug, Default)]
struct Worker {
    ratio: f64,
    quality: i32,
    wage: i32,
}

impl Eq for Worker {}

impl PartialEq for Worker {
    fn eq(&self, other: &Self) -> bool {
        (self.ratio - other.ratio).abs() < EPSILON
    }
}

impl PartialOrd for Worker {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Worker {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.ratio - other.ratio > EPSILON {
            Ordering::Greater
        } else if other.ratio - self.ratio > EPSILON {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        use std::collections::BinaryHeap;

        let n = quality.len();
        let mut workers = vec![];
        for i in 0..n {
            let worker = Worker {
                ratio: wage[i] as f64 / quality[i] as f64,
                quality: quality[i],
                wage: wage[i],
            };
            workers.push(worker);
        }
        workers.sort();
        let mut ans = 1.0e9;
        let mut sum = 0;
        let mut heap = BinaryHeap::<i32>::new();
        for worker in &workers {
            heap.push(worker.quality);
            sum += worker.quality;
            if heap.len() as i32 > k {
                sum -= heap.pop().unwrap();
            }
            if heap.len() as i32 == k {
                let x = sum as f64 * worker.ratio;
                if x < ans {
                    ans = x
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mincost_to_hire_workers() {
        assert_eq!(
            Solution::mincost_to_hire_workers(vec![10, 20, 5], vec![70, 50, 30], 2),
            105.0
        );
    }

    #[test]
    fn test_mincost_to_hire_workers_02() {
        assert_eq!(
            (Solution::mincost_to_hire_workers(vec![3, 1, 10, 10, 1], vec![4, 8, 2, 2, 7], 3)
                - 30.6666666666666667)
                .abs()
                < 1.0e-5,
            true
        );
    }
}