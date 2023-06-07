// https://leetcode.com/problems/number-of-ways-to-stay-in-the-same-place-after-some-steps/solutions/3527268/rust-iterator-dp-bottom-up-space-optimized/
impl Solution {
    pub fn num_ways(steps: i32, arr_len: i32) -> i32 {
        let k = steps as usize;
        let n = k.min(arr_len as usize);
        (0..k)
        .fold(
            {
                let mut memo: Vec<usize> = vec![0; n];
                memo[0] = 1;
                memo
            },
            |memo, _| {
                (0..n).map(|i| {
                    let mut ways = memo[i];
                    if i > 0 {
                        ways = (ways + memo[i-1]) % 1_000_000_007;
                    }
                    if i+1 < n {
                        ways = (ways + memo[i+1]) % 1_000_000_007;
                    }
                    ways
                })
                .collect()
        })
        .get(0)
        .unwrap_or(&0)
        .clone() as i32
    }
}