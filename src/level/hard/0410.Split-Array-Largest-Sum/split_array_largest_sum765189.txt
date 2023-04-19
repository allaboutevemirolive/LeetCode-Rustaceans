// https://leetcode.com/problems/split-array-largest-sum/solutions/765189/rust-translated-0ms/
impl Solution {
    pub fn split_array(nums: Vec<i32>, m: i32) -> i32 {
        fn valid(target: i64, v: &[i32], m: i32) -> bool {
            let mut count = 1;
            let mut total = 0i64;
            for &x in v {
                total += x as i64;
                if total > target {
                    total = x as i64;
                    count += 1;
                    if count > m {
                        return false;
                    }
                }
            }
            true
        }

        let mut max = 0;
        let mut sum = 0i64;
        for &x in &nums {
            max = std::cmp::max(max, x);
            sum += x as i64;
        }
        if m == 1 {
            return sum as i32;
        };
        let mut left = max as i64;
        let mut right = sum;

        while left <= right {
            let mid = (left + right) / 2;
            if valid(mid, &nums, m) {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        left as i32        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_array() {
        assert_eq!(Solution::split_array(vec![7, 2, 5, 10, 8], 2), 18)
    }
}