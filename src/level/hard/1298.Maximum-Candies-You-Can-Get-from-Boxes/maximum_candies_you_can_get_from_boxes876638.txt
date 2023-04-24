// https://leetcode.com/problems/maximum-candies-you-can-get-from-boxes/solutions/876638/rust-translated-16ms-100/
impl Solution {
    pub fn max_candies(
        mut status: Vec<i32>,
        candies: Vec<i32>,
        keys: Vec<Vec<i32>>,
        contained_boxes: Vec<Vec<i32>>,
        initial_boxes: Vec<i32>,
    ) -> i32 {
        use std::collections::VecDeque;

        let mut ans = 0;
        let mut q = VecDeque::<i32>::new();
        for i in initial_boxes {
            status[i as usize] += 5000;
            if status[i as usize] > 5000 {
                q.push_back(i as i32);
            }
        }
        while let Some(b) = q.pop_front() {
            ans += candies[b as usize];
            for &i in &keys[b as usize] {
                status[i as usize] += 5;
                if status[i as usize] == 5005 {
                    q.push_back(i as i32);
                }
            }
            for &i in &contained_boxes[b as usize] {
                status[i as usize] += 5000;
                if status[i as usize] > 5000 {
                    q.push_back(i as i32);
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
    fn test_max_candies() {
        assert_eq!(
            Solution::max_candies(
                vec![1, 0, 1, 0],
                vec![7, 5, 4, 100],
                vec![vec![], vec![], vec![1], vec![]],
                vec![vec![1, 2], vec![3], vec![], vec![]],
                vec![0]
            ),
            16
        );
    }

    #[test]
    fn test_max_candies_02() {
        assert_eq!(
            Solution::max_candies(
                vec![1, 0, 0, 0, 0, 0],
                vec![1, 1, 1, 1, 1, 1],
                vec![vec![1, 2, 3, 4, 5], vec![], vec![], vec![], vec![], vec![]],
                vec![vec![1, 2, 3, 4, 5], vec![], vec![], vec![], vec![], vec![]],
                vec![0]
            ),
            6
        );
    }

    #[test]
    fn test_max_candies_03() {
        assert_eq!(
            Solution::max_candies(
                vec![1, 1, 1],
                vec![100, 1, 100],
                vec![vec![], vec![0, 2], vec![]],
                vec![vec![], vec![], vec![]],
                vec![1]
            ),
            1
        );
    }

    #[test]
    fn test_max_candies_04() {
        assert_eq!(
            Solution::max_candies(vec![1], vec![100], vec![vec![]], vec![vec![]], vec![]),
            0
        );
    }

    #[test]
    fn test_max_candies_05() {
        assert_eq!(
            Solution::max_candies(
                vec![1, 1, 1],
                vec![2, 3, 2],
                vec![vec![], vec![], vec![]],
                vec![vec![], vec![], vec![]],
                vec![2, 1, 0]
            ),
            7
        );
    }
}