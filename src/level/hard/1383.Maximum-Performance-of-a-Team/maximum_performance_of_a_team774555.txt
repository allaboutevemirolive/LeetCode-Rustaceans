// https://leetcode.com/problems/maximum-performance-of-a-team/solutions/774555/rust-translated/
impl Solution {
    pub fn max_performance(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
        use std::collections::{BinaryHeap, HashMap};

        let mut v: Vec<(i32, i32)> = efficiency
            .iter()
            .zip(speed.iter())
            .map(|(&e, &s)| (-e, -s))
            .collect();
        v.sort();
        //       v.sort_by_key(|x| { -x.0 });
        //        println!("{:?}", v);
        let mut heap = BinaryHeap::<i32>::new();
        let mut ans = 0i64;
        let mut sum = 0i64;
        for (e, s) in v {
            heap.push(s);
            sum += s as i64;
            if heap.len() as i32 > k {
                sum -= heap.pop().unwrap() as i64
            }
            ans = std::cmp::max(ans, sum * e as i64);
            //            println!("sum = {}, e = {}, s = {}", sum, e, s);
        }
        (ans % 1_000_000_007) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_performance() {
        assert_eq!(
            Solution::max_performance(6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 2),
            60
        )
    }

    #[test]
    fn test_max_performance_02() {
        assert_eq!(
            Solution::max_performance(6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 3),
            68
        )
    }

    #[test]
    fn test_max_performance_03() {
        assert_eq!(
            Solution::max_performance(6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 4),
            72
        )
    }
}