// https://leetcode.com/problems/bitwise-and-of-numbers-range/solutions/603111/two-rust-solutions-both-0ms/
macro_rules! log_of {
    ($val:expr, $base:expr, $type:ty) => {
         ($val as f32).log($base) as $type
    }
}

impl Solution {
    pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
        if (log_of!(m, 2., usize) != log_of!(n, 2., usize)) {
            return 0;
        } else if m == n {
            return m;
        } else {
            let mut res = m;
            for i in m+1..=n {
                res &= i;
            }
            return res;
        }
    }
}