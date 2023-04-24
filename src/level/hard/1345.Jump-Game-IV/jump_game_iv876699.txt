// https://leetcode.com/problems/jump-game-iv/solutions/876699/rust-translated-32ms-100/
impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        use std::collections::{HashMap, VecDeque};

        let n = arr.len();
        let mut graph = HashMap::<i32, Vec<i32>>::new();
        for i in 0..n {
            graph.entry(arr[i as usize]).or_default().push(i as i32);
        }
        let mut visited = vec![false; n];
        visited[0] = true;
        let mut q = VecDeque::<i32>::new();
        q.push_back(0);
        let mut step = 0;
        while !q.is_empty() {
            let size = q.len();
            for _ in 0..size {
                let i = q.pop_front().unwrap();
                if i == n as i32 - 1 {
                    return step;
                }
                let next = graph.get_mut(&arr[i as usize]).unwrap();
                next.push(i - 1);
                next.push(i + 1);
                for &j in next.iter() {
                    if j >= 0 && j < n as i32 && !visited[j as usize] {
                        visited[j as usize] = true;
                        q.push_back(j);
                    }
                }
                next.clear()
            }
            step += 1;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_jumps() {
        assert_eq!(
            Solution::min_jumps(vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404]),
            3
        );
    }

    #[test]
    fn test_min_jumps_02() {
        assert_eq!(Solution::min_jumps(vec![7]), 0);
    }

    #[test]
    fn test_min_jumps_03() {
        assert_eq!(Solution::min_jumps(vec![7, 6, 9, 6, 9, 6, 9, 7]), 1);
    }

    #[test]
    fn test_min_jumps_04() {
        assert_eq!(Solution::min_jumps(vec![6, 1, 9]), 2);
    }

    #[test]
    fn test_min_jumps_05() {
        assert_eq!(
            Solution::min_jumps(vec![11, 22, 7, 7, 7, 7, 7, 7, 7, 22, 13]),
            3
        );
    }
}