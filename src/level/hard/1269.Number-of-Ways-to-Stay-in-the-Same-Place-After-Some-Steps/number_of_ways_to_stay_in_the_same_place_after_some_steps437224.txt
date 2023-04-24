// https://leetcode.com/problems/number-of-ways-to-stay-in-the-same-place-after-some-steps/solutions/437224/rust-rolling-array-dp/
impl Solution {
    pub fn num_ways(steps: i32, arr_len: i32) -> i32 {
        let md = 1000000007;
        let mut ans = [[0; 256]; 2];
        ans[0][0] = 1;
        for i in 0i32..steps {
            ans[(i & 1 ^ 1) as usize] = [0; 256];
            for j in 0i32..251 {
                if ans[(i & 1) as usize][j as usize] == 0 {
                    break;
                }
                for jj in &[j - 1, j, j + 1] {
                    if *jj >= 0 && *jj < arr_len {
                        ans[(i & 1 ^ 1) as usize][*jj as usize] += ans[(i & 1) as usize][j as usize];
                        ans[(i & 1 ^ 1) as usize][*jj as usize] %= md;
                    }
                }
            }
        }

        ans[(steps & 1) as usize][0]
    }
}