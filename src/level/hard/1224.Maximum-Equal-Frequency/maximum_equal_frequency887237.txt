// https://leetcode.com/problems/maximum-equal-frequency/solutions/887237/rust-translated-8ms-100/
impl Solution {
    pub fn max_equal_freq(nums: Vec<i32>) -> i32 {
        let mut count = vec![0i32; 100_001];
        let mut freq = vec![0i32; 100_001];
        let n = nums.len();
        let mut ans = 0;
        for i in 1..=n {
            let a = nums[i - 1];
            let mut c = count[a as usize];
            freq[c as usize] -= 1;
            c += 1;
            count[a as usize] = c;
            freq[c as usize] += 1;
            if freq[c as usize] * c == i as i32 && i < n {
                ans = i as i32 + 1;
            }
            let d = i as i32 - freq[c as usize] * c;
            if (d == c + 1 || d == 1) && freq[d as usize] == 1 {
                ans = i as i32;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_equal_freq() {
        assert_eq!(Solution::max_equal_freq(vec![2, 2, 1, 1, 5, 3, 3, 5]), 7);
    }

    #[test]
    fn test_max_equal_freq_02() {
        assert_eq!(
            Solution::max_equal_freq(vec![1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5]),
            13
        );
    }

    #[test]
    fn test_max_equal_freq_03() {
        assert_eq!(Solution::max_equal_freq(vec![1, 1, 1, 2, 2, 2]), 5);
    }
    #[test]
    fn test_max_equal_freq_04() {
        assert_eq!(
            Solution::max_equal_freq(vec![10, 2, 8, 9, 3, 8, 1, 5, 2, 3, 7, 6]),
            8
        );
    }
}