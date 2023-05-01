// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/solutions/2130260/rust-solution/
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut end = numbers.len() - 1;
        if target > 0 {
            if let Ok(i) = numbers.binary_search(&target) {
                end = i;
            }
        }

        loop {
            let val = target - numbers[end];
            match numbers[..end].binary_search(&val) {
                Ok(i) => break vec![i as i32 + 1, end as i32 + 1],
                _ => end -= 1,
            }
        }
    }
}