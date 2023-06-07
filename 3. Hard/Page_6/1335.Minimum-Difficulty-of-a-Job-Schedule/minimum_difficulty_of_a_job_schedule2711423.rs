// https://leetcode.com/problems/minimum-difficulty-of-a-job-schedule/solutions/2711423/python-rust-bottom-up-and-top-down-dp-solutions-with-detailed-comments/
impl Solution 
{
    pub fn min_difficulty(job_diff: Vec<i32>, days: i32) -> i32 
    {
        let jobs = job_diff.len() as usize;
        let days = days as usize;

        // [1] trivial case
        if days > jobs { return -1; }

        // [2] as a collection of state variables for bottom-up DP, we
        //     use a matrix where for each job and for each day we compute
        //     the minimal possible schedule difficulty that is attainable
        //     when considering all following days (yes, we iterate in 
        //     reverse order, thus, reproducing in some sense the logic
        //     of the Python top-down solution above) 
        let mut schedule_diff = vec![vec![i32::MAX;days+1];jobs+1];
        schedule_diff[jobs][days] = 0;
        
        // [3] iteratively update DP state starting from the last day  
        //     and last job till we reach first day and first job;
        //     when considering some day, we have to keep track of 
        //     indices such that there is always room for at least one
        //     job in each of preceding and following days;
        for d in (0..days).rev()
        {
            let last_idx = jobs - days + d;
            for j in (d..=last_idx).rev()
            {
                let mut this_day_diff = 0;
                for i in (j..=last_idx)
                {
                    this_day_diff = this_day_diff.max(job_diff[i]);
                    if schedule_diff[i+1][d+1] < i32::MAX
                    {
                        // [4] magic happens here: for each day 'd' and each job 'j', we test
                        //     whether the cumulative difficulty for jobs [j,i] on day 'd' 
                        //     together with jobs [i+1,n) on days [d+1,days) is less than
                        //     the currently attained value for some other 'i'
                        schedule_diff[j][d] = schedule_diff[j][d].min(this_day_diff + schedule_diff[i+1][d+1]);
                    }
                }
            }
        }

        // [5] in the end, the minimal attainable schedule difficulty
        //     will be stored as the DP state for day=0 and job=0
        return schedule_diff[0][0];
    }
}