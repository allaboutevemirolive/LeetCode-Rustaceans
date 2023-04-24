// https://leetcode.com/problems/maximum-profit-in-job-scheduling/solutions/1150815/rust-top-down-dp/
impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut slots: Vec<(i32, i32, i32)> = start_time
            .into_iter()
            .zip(end_time.into_iter())
            .zip(profit.into_iter())
            .map(|((s, e), p)| (s, e, p))
            .collect();
        slots.sort_by(|x, y| x.0.cmp(&y.0));
        // NOTE: cache best profit start from index
        let mut cache: Vec<Option<i32>> = vec![None; slots.len()];
        Self::dfs(&slots, 0, &mut cache)
    }

    fn dfs(slots: &[(i32, i32, i32)], cur: usize, cache: &mut Vec<Option<i32>>) -> i32 {
        if cur >= slots.len() {
            return 0;
        }

        if let Some(cached) = cache[cur] {
            return cached;
        }

        // for every time slot, one can choose do or skip.
        // if do the job, get the profit, move cursor to the next possbile start_time, dfs again
        // else, dfs next time slot directly

        // NOTE: do the job
        let res1 = {
            let mut res = slots[cur].2;
            for i in (cur + 1)..slots.len() {
                if slots[i].0 >= slots[cur].1 {
                    res += Self::dfs(slots, i, cache);
                    break;
                }
            }
            res
        };
        // NOTE: skip the job
        let res2 = Self::dfs(slots, cur + 1, cache);

        let res = std::cmp::max(res1, res2);
        cache[cur] = Some(res);
        res
    }
}