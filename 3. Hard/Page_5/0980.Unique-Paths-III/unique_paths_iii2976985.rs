// https://leetcode.com/problems/unique-paths-iii/solutions/2976985/short-and-fast-rust-solution-dfs-dp/
use std::collections::*;

impl Solution {
    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        let (n, m, mut mask) = (grid.len(), grid[0].len(), 0);
        let (mut dp, mut start, mut end) = (HashMap::from([((0, 0), m as i32+2)]), 0, 0);
        for j in 0..m { mask|=1<<j; mask|=1<<(n+1)*(m+2)+j; }
        for i in 0..=n { mask|=3<<(m+i*(m+2)); }
        for (i,x) in grid.iter().enumerate() {
            for (j,&y) in x.iter().enumerate() { let t = 1<<(i+1)*(m+2)+j;
                if (y<0) { mask |= t; } else if y==1 { start = t; } else if y==2 { end = t; }
            }
        }

        fn rec(curr:usize, mask: usize, end: usize, dp: &mut HashMap<(usize, usize), i32>) ->i32 {
            if curr==end { return (64-mask.leading_zeros()==mask.count_ones()) as i32; }
            if dp.contains_key(&(curr, mask)) { return *dp.get(&(curr, mask)).unwrap(); }
            let mut ans = 0;
            if mask & (curr<<dp[&(0,0)]) == 0 { ans+=rec(curr<<dp[&(0,0)], mask|(curr<<dp[&(0,0)]), end, dp); }
            if mask & (curr<<1) == 0 { ans+=rec(curr<<1, mask|(curr<<1), end, dp); }
            if mask & (curr>>dp[&(0,0)]) == 0 { ans+=rec(curr>>dp[&(0,0)], mask|(curr>>dp[&(0,0)]), end, dp); }
            if mask & (curr>>1) == 0 { ans+=rec(curr>>1, mask|(curr>>1), end, dp); } 
            dp.insert((curr, mask), ans); ans
        }
        rec(start, mask|start, end, &mut dp)
    }
}