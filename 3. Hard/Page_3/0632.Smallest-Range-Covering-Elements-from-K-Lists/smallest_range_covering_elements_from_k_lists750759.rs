// https://leetcode.com/problems/smallest-range-covering-elements-from-k-lists/solutions/750759/rust-translated-two-pointers/
impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let n = nums.len();
        let mut points = vec![];
        for i in 0..n {
            for &j in &nums[i] {
                points.push((j as i32, i as i32))
            }
        }
        points.sort_by(|a, b| a.0.cmp(&b.0));

        let mut counts = vec![0; n];
        let mut count_unique = 0;
        let mut min_start = -1;
        let mut min_len = std::i32::MAX;
        let mut i = 0;
        for j in 0..points.len() {
            if counts[points[j].1 as usize] == 0 {
                count_unique += 1;
            }
            counts[points[j].1 as usize] += 1;
            while count_unique == n {
                if points[j].0 - points[i].0 + 1 < min_len {
                    min_start = points[i].0;
                    min_len = points[j].0 - points[i].0 + 1;
                }
                let mut prev = points[i].0;
                while i <= j && prev == points[i].0 {
                    counts[points[i].1 as usize] -= 1;
                    if counts[points[i].1 as usize] == 0 {
                        count_unique -= 1
                    }
                    i += 1;
                }
            }
        }
        vec![min_start, min_start + min_len - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smallest_range() {
        assert_eq!(
            Solution::smallest_range(vec![
                vec![4, 10, 15, 24, 26],
                vec![0, 9, 12, 20],
                vec![5, 18, 22, 30]
            ]),
            vec![20, 24]
        )
    }
}