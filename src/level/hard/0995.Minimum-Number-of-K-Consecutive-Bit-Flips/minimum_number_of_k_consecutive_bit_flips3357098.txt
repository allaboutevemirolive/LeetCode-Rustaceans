// https://leetcode.com/problems/minimum-number-of-k-consecutive-bit-flips/solutions/3357098/rust-fast-and-simple-15ms/
impl Solution {
    pub fn min_k_bit_flips(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize; 
        let n = nums.len(); 
        let mut res = 0;
        let mut q = std::collections::VecDeque::with_capacity(k);
        for (i, v) in nums.iter().enumerate() {
            if let Some(&j) = q.front() { 
                if j == i {
                    q.pop_front();
                }
            }
            if (v + q.len() as i32) & 1 == 0 {
                if i > n - k {
                    return -1
                } else {
                    q.push_back(i + k);
                    res += 1;
                }
            }
        }
        res
    }
}