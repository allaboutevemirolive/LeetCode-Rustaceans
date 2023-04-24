// https://leetcode.com/problems/maximum-profit-in-job-scheduling/solutions/475345/rust-o-n-logn-15-lines/
    pub fn job_scheduling(start_time: Vec<i32>,
                          end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let (mut dp, mut arr) = (vec![0i32; start_time.len()], (0..start_time.len())
            .map(|i| (start_time[i], end_time[i], profit[i])).collect::<Vec<_>>());
        arr.sort_by_key(|x| x.1);

        for i in 0..start_time.len() {
            let extra = match arr[..i].binary_search_by(|probe| probe.1.cmp(&arr[i].0)) {
                Ok(mut pos) => {
                    while pos+1 < i && arr[pos+1].1 == arr[i].0 { pos += 1 }
                    dp[pos]
                },
                Err(pos) => if pos == 0 { 0 } else { dp[pos-1] },
            };
            dp[i] = std::cmp::max(if i > 0 { dp[i-1] } else { arr[i].2 },
                                  arr[i].2 + extra);
        }
        *dp.last().unwrap_or(&0)
    }