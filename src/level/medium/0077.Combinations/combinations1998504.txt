// https://leetcode.com/problems/combinations/solutions/1998504/rust-iterative-preallocation-4-ms/
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let k = k as usize;
        let factorial = |n| (1..=n as u64).fold(1, |acc, x| acc*x) as usize;
        let mut rez = vec![vec![0; k]; factorial(n)/factorial(k)/factorial(n-k)];

        let mut nums = (1..=k as i32).chain(std::iter::once(n as i32+1)).collect::<Vec<_>>();
        let mut i = 0;
        let mut j = 0;
        while j < k {
            rez[i].copy_from_slice(&nums[0..k]);
            i += 1;
            j = 0;
            while j < k && nums[j+1] == nums[j] + 1 {
                nums[j] = j as i32 + 1;
                j += 1;
            }
            nums[j] += 1;
        }
        rez
    }
}