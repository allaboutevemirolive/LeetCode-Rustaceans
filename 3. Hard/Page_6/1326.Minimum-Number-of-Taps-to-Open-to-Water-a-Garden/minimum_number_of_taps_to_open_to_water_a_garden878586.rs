// https://leetcode.com/problems/minimum-number-of-taps-to-open-to-water-a-garden/solutions/878586/rust-translated-4ms-100/
impl Solution {
    pub fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
        let mut dp = vec![n + 2; n as usize + 1];
        dp[0] = 0;
        for i in 0..=n {
            for j in std::cmp::max(0, i - ranges[i as usize] + 1)
                ..=std::cmp::min(i + ranges[i as usize], n)
            {
                dp[j as usize] = std::cmp::min(
                    dp[j as usize],
                    dp[std::cmp::max(0, i - ranges[i as usize]) as usize] + 1,
                );
            }
        }
        if dp[n as usize] < n + 2 {
            dp[n as usize]
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_taps() {
        assert_eq!(Solution::min_taps(5, vec![3, 4, 1, 1, 0, 0]), 1);
    }

    #[test]
    fn test_min_taps_02() {
        assert_eq!(Solution::min_taps(3, vec![0, 0, 0, 0]), -1);
    }

    #[test]
    fn test_min_taps_03() {
        assert_eq!(Solution::min_taps(8, vec![4, 0, 0, 0, 0, 0, 0, 0, 4]), 2);
    }
    #[test]
    fn test_min_taps_04() {
        assert_eq!(Solution::min_taps(8, vec![4, 0, 0, 0, 4, 0, 0, 0, 4]), 1);
    }

    #[test]
    fn test_min_taps_05() {
        assert_eq!(Solution::min_taps(7, vec![1, 2, 1, 0, 2, 1, 0, 1]), 3);
    }
}