// https://leetcode.com/problems/kth-smallest-number-in-multiplication-table/solutions/1580740/rust-solution/
impl Solution {
    pub fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
        let mut l = 0;
        let mut r = m * n + 1;
        while l + 1 < r {
            let mid = l + (r - l) / 2;
            if (1..=m).map(|x| (mid / x).min(n)).sum::<i32>() < k {
                l = mid;
            } else {
                r = mid;
            }
        }
        r
    }
}