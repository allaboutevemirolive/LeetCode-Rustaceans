// https://leetcode.com/problems/kth-smallest-number-in-multiplication-table/solutions/1581087/rust-clean-simple-binary-search-solution-beats-100/
impl Solution {
    pub fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
        let (m, n) = (m.min(n), m.max(n));
        let (mut l, mut r) = (1, n * m);
        while l < r {
            let mid = (l + r) / 2;
            let mut rank = 0;
            for i in 1..m + 1 {
                rank += n.min(mid / i);
            }
            if rank < k { l = mid + 1; }
            else { r = mid; }
        }
        l
    }
}