// https://leetcode.com/problems/maximum-profit-in-job-scheduling/solutions/2851923/rust-o-n-log-n-10ms/
impl Solution {
    pub fn job_scheduling(start_time : Vec<i32>, 
                          end_time   : Vec<i32>, 
                          profit     : Vec<i32>) 
        -> i32 
    {
        let n = start_time.len();
        let mut table = vec![0; n];
        let mut jobs = JobList::new(&start_time, &end_time, &profit);

        jobs.sort_by_end_time();

        table[0] = jobs.profit(0);

        for i in 1..n {
            match jobs.find_by_end_time(&jobs.start(i)) {
                Ok(j) => {
                    table[i] = table[i - 1].max(jobs.profit(i) + table[j]);
                },
                Err(j) if j > 0 => {
                    table[i] = table[i - 1].max(jobs.profit(i) + table[j - 1]);
                },
                _ => {
                    table[i] = table[i - 1].max(jobs.profit(i));
                },
            }
        }

        table[n - 1]
    }

}