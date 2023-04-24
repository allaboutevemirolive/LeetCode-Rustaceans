// https://leetcode.com/problems/constrained-subsequence-sum/solutions/599573/rust-solution-8ms-with-detailed-explanation/
impl Solution {
    pub fn constrained_subset_sum(A: Vec<i32>, k: i32) -> i32 {
        let mut ns = A.to_vec();
        let mut d: VecDeque<i32> = VecDeque::new();
        let mut res = ns[0];

        for i in (0..A.len()) {
            ns[i] += if d.is_empty() { 0 } else { d[0] };
            res = max(res, ns[i]);
            while !d.is_empty() && d.back().unwrap() < &ns[i] {
                d.pop_back();
            }
            if ns[i] > 0 {
                d.push_back(ns[i])
            };
            if i >= k as usize && !d.is_empty() && d[0] == ns[i - k as usize] {
                d.pop_front();
            }
        }
        res
    }
}