// https://leetcode.com/problems/non-negative-integers-without-consecutive-ones/solutions/803192/rust-0ms-2-1m/
impl Solution {
    pub fn find_integers(num: i32) -> i32 {
        let mut f = [0; 32];
        f[0] = 1;
        f[1] = 2;
        for i in 2..32 {
            f[i] = f[i - 1] + f[i - 2];
        }
        let mut i = 30i32;
        let mut sum = 0;
        let mut prev_bit = 0;
        while i >= 0 {
            if (num & (1 << i)) != 0 {
                sum += f[i as usize];
                if prev_bit == 1 {
                    sum -= 1;
                    break;
                }
                prev_bit = 1;
            } else {
                prev_bit = 0;
            }
            i -= 1;
        }
        sum + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_integers() {
        assert_eq!(Solution::find_integers(5), 5)
    }
}