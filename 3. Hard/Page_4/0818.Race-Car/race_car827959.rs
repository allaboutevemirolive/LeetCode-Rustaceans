// https://leetcode.com/problems/race-car/solutions/827959/rust-translated-0ms-100/
impl Solution {
    pub fn racecar(target: i32) -> i32 {
        fn number_of_leading_zeros(x: i32) -> i32 {
            let mut count = 0;
            let mut mask = -2147483648i32;
            while x & mask == 0 {
                count += 1;
                mask >>= 1;
            }
            count
        }

        let mut dp = vec![std::i32::MAX; target as usize + 4];
        dp[0] = 0;
        dp[1] = 1;
        dp[2] = 4;
        dp[3] = 2;
        for t in 3..=target {
            let k = 32 - number_of_leading_zeros(t);
            if t == (1 << k) - 1 {
                dp[t as usize] = k;
                continue;
            }
            for j in 0..k - 1 {
                dp[t as usize] = std::cmp::min(
                    dp[t as usize],
                    dp[t as usize - (1 << (k - 1)) + (1 << j)] + k - 1 + j + 2,
                );
            }
            if (1 << k) - 1 - t < t {
                dp[t as usize] =
                    std::cmp::min(dp[t as usize], dp[(1 << k) - 1 - t as usize] + k + 1);
            }
        }
        dp[target as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_racecar() {
        assert_eq!(Solution::racecar(3), 2)
    }

    #[test]
    fn test_racecar_02() {
        assert_eq!(Solution::racecar(6), 5)
    }

    #[test]
    fn test_racecar_03() {
        assert_eq!(Solution::racecar(7), 3)
    }

    #[test]
    fn test_racecar_04() {
        assert_eq!(Solution::racecar(15), 4)
    }

    #[test]
    fn test_racecar_05() {
        assert_eq!(Solution::racecar(65535), 16)
    }
}