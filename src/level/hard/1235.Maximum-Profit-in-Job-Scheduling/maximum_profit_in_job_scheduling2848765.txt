// https://leetcode.com/problems/maximum-profit-in-job-scheduling/solutions/2848765/rust-elixir-sorting-binary-search/
impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut jobs: Vec<usize> = (0..profit.len()).collect();
        jobs.sort_unstable_by_key(|&i| end_time[i]);
        let mut v: Vec<(i32, i32)> = vec![(0, i32::MIN)];
        for &i in jobs.iter() {
            let left = match v.binary_search_by_key(&start_time[i], |&(_, t)| t) {
                Ok(j) => j,
                Err(j) => j - 1,
            };
            if v[left].0 + profit[i] > v.last().unwrap().0 {
                if end_time[i] == v.last().unwrap().1 {
                    v.last_mut().unwrap().0 = v[left].0 + profit[i];
                }
                else {
                    v.push((v[left].0 + profit[i], end_time[i]));
                }
            }
        }
        v.last().unwrap().0
    }
}