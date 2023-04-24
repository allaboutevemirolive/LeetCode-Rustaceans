// https://leetcode.com/problems/construct-target-array-with-multiple-sums/solutions/878545/rust-translated-0ms-100/
impl Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        use std::collections::BinaryHeap;

        let mut sum = 0i64;
        let mut heap = BinaryHeap::<i32>::new();
        for x in target {
            heap.push(x);
            sum += x as i64;
        }
        while let Some(mut x) = heap.pop() {
            sum -= x as i64;
            if x == 1 || sum == 1 {
                return true;
            }
            if (x as i64) < sum || sum == 0 || (x as i64) % sum == 0 {
                return false;
            }
            x = ((x as i64) % sum) as i32;
            sum += x as i64;
            heap.push(x);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_possible() {
        assert_eq!(Solution::is_possible(vec![9, 3, 5]), true);
    }

    #[test]
    fn test_is_possible_02() {
        assert_eq!(Solution::is_possible(vec![1, 1, 1, 2]), false);
    }

    #[test]
    fn test_is_possible_03() {
        assert_eq!(Solution::is_possible(vec![8, 5]), true);
    }
}