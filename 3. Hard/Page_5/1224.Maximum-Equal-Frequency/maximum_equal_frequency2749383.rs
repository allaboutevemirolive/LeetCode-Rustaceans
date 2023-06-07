// https://leetcode.com/problems/maximum-equal-frequency/solutions/2749383/rust-solution/
//https://leetcode.com/problems/maximum-equal-frequency/

#[allow(unused)]
const M: i64 = 1_000_000_000 + 7;

#[cfg(local)]
struct Solution {}

#[allow(unused)]
impl Solution {
    pub fn max_equal_freq(nums: Vec<i32>) -> i32 {
        let mut cnt_histogram = [0; 100010]; // how many we have from a given cnt
        let mut num_histogram = [0; 100010]; // number of times we saw a given number
        let mut cnt_min = 0;
        let mut cnt_max = 0;
        let mut max_len: i32 = 0;

        nums.into_iter().enumerate().for_each(|(idx, v)| {
            let v = v as usize;
            let mut c = num_histogram[v];
            if c > 0 {
                cnt_histogram[c] -= 1;
                if c == cnt_min && cnt_histogram[c] == 0 {
                    cnt_min += 1;
                }
            } else {
                cnt_min = 1;
            }
            c += 1;
            cnt_histogram[c] += 1;
            if c > cnt_max {
                cnt_max = c;
            }
            num_histogram[v] += 1;

            let n = idx + 1;

            let s_simple = { cnt_max == 1 };
            let s_single = (cnt_min == 1 && n == cnt_histogram[cnt_max] * cnt_max + 1);
            let s_over = {
                (cnt_max > 1
                    && cnt_histogram[cnt_max] == 1
                    && n == cnt_histogram[cnt_max - 1] * (cnt_max - 1) + cnt_max)
            };
            if s_simple || s_single || s_over {
                max_len = n as i32;
            }
        });
        max_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::max_equal_freq(vec![2, 2, 1, 1, 5, 3, 3, 5]), 7);
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::max_equal_freq(vec![1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5]),
            13
        );
    }
    #[test]
    fn test3_wa() {
        assert_eq!(Solution::max_equal_freq(vec![1, 1, 1, 2, 2, 2]), 5);
    }
}