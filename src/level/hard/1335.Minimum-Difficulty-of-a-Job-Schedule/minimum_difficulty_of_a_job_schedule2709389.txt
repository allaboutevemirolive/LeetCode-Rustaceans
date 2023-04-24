// https://leetcode.com/problems/minimum-difficulty-of-a-job-schedule/solutions/2709389/rust-solution/
impl Solution {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let n = job_difficulty.len();
        if n < d as usize {
            return -1;
        }
        let mut a = vec![i32::MAX; n + 1];
        let mut b = vec![i32::MAX; n + 1];
        a[0] = 0;
        for _ in 0..d {
            for (i, val) in b.iter_mut().enumerate() {
                *val = job_difficulty[0..i]
                    .iter()
                    .zip(&a)
                    .rev()
                    .scan(0, |c, (&x, s)| {
                        *c = (*c).max(x);
                        Some(s.saturating_add(*c))
                    })
                    .min()
                    .unwrap_or(i32::MAX);
            }
            std::mem::swap(&mut a, &mut b);
        }
        a.pop().unwrap()
    }
}