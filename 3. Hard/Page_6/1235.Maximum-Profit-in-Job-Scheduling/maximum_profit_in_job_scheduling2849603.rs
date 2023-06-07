// https://leetcode.com/problems/maximum-profit-in-job-scheduling/solutions/2849603/rust-topdown-dp/
pub fn job_scheduling(
    start_time: impl AsRef<[i32]>,
    end_time: impl AsRef<[i32]>,
    profit: impl AsRef<[i32]>,
) -> i32 {
    let st = start_time.as_ref();
    let et = end_time.as_ref();
    let pr = profit.as_ref();

    let mut data = st
        .iter()
        .copied()
        .zip(et.iter().copied())
        .zip(pr.iter().copied())
        .map(|((a, b), c)| (a, b, c))
        .collect::<Vec<_>>();
    data.sort_unstable_by_key(|&(a, b, c)| (a, b, -c));

    let mut cache = vec![-1; st.len()];
    dfs(&mut cache, &data, 0, 0)
}

fn dfs(cache: &mut [i32], data: &[(i32, i32, i32)], from: usize, start_time: i32) -> i32 {
    if from >= data.len() {
        return 0;
    }

    // Do not re-calculate already processed trees
    if cache[from] >= 0 {
        return cache[from];
    }

    // Calculate the profit if we take the job at index 'from'
    let take = dfs(
        cache,
        data,
        next_job_idx(&data, from + 1, data[from].1),
        data[from].1,
    ) + data[from].2;

    //  Calculate the profit if we skip the job at index 'from'
    let skip = dfs(cache, data, from + 1, start_time);

    // Take the most profitable action
    let profit = take.max(skip);
    cache[from] = profit;
    profit
}

fn next_job_idx(data: &[(i32, i32, i32)], from: usize, st: i32) -> usize {
    let mut lo = from;
    let mut hi = data.len();

    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if data[mid].0 >= st {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }

    lo
}