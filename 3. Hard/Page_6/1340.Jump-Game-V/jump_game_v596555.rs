// https://leetcode.com/problems/jump-game-v/solutions/596555/rust-dfs-with-memorization-8ms-2-3-mb/
impl Solution {
    pub fn max_jumps(arr: Vec<i32>, d: i32) -> i32 {
        let mut memo: Vec<i32> = vec![0; arr.len()];
        arr.iter()
            .enumerate()
            .map(|(idx, num)| dfs(&arr, &mut memo, d as usize, idx))
            .max()
            .unwrap()
    }
}

fn dfs(arr: &Vec<i32>, memo: &mut Vec<i32>, max_allowed: usize, idx: usize) -> i32 {
    if memo[idx] != 0 {
        return memo[idx];
    }

    let result = {
        let mut left = idx;
        let mut right = idx;
        let mut mx = 1;

        while left > 0 && idx - left < max_allowed && arr[left - 1] < arr[idx] {
            left -= 1;
            mx = mx.max(dfs(arr, memo, max_allowed, left) + 1);
        }

        while right + 1 < arr.len() && right - idx < max_allowed && arr[right + 1] < arr[idx] {
            right += 1;
            mx = mx.max(dfs(arr, memo, max_allowed, right) + 1);
        }

        mx
    };
    memo[idx] = result;
    result
}