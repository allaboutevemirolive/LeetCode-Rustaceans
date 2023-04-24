// https://leetcode.com/problems/minimum-cost-to-merge-stones/solutions/428620/rust-dp-solution-aspired-by-lee215/
///思路，石头数需满足(k-1)*n + k = len，否则认为无法移动。
/// 适合用动态规划来进行解决
/// dp(i,j,m)定义为将j-i块石头，每次移动k块，最后剩下m块
/// 此题相当于 dp(0,len,1)
/// 如果不满足 len - a*(k-1)=m return -1
/// 如果 m = 1 dp(i,j,1) = dp(i,j,k-1)+ sum(i,j);
/// dp(0,len,m) = min(dp(0,mid,1) + dp(mid,len,m-1)) ,如果有-1 直接等于 -1
impl Solution {
    pub fn merge_stones(stones: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;
        use std::f64::INFINITY;
        let len = stones.len();
        let mut records = HashMap::new();
        pub fn dp(
            i: i32,
            j: i32,
            m: i32,
            k: i32,
            stones: &Vec<i32>,
            records: &mut HashMap<(i32, i32, i32), f64>,
        ) -> f64 {
            let key = (i, j, m);
            if !records.contains_key(&key) {
                let new_value = if (j - i - m) % (k - 1) != 0 {
                    INFINITY
                } else if j - i == m {
                    0.0
                } else if j - i < m {
                    INFINITY
                } else if m == 1 {
                    dp(i, j, k, k, &stones, records)
                        + stones[i as usize..j as usize].iter().sum::<i32>() as f64
                } else {
                    ((i + 1)..j)
                        .step_by(k as usize - 1)
                        .map(|mid| {
                            dp(i, mid, 1, k, stones, records)
                                + dp(mid, j, m - 1, k, stones, records)
                        })
                        .min_by(|a, b| a.partial_cmp(b).unwrap())
                        .unwrap_or(INFINITY)
                };
                records.insert(key, new_value);
            }
            records.get(&(i, j, m)).unwrap().to_owned()
        }
        let result = dp(0, len as i32, 1, k, &stones, &mut records);
        dbg!(&result);
        if result == INFINITY {
            -1
        } else {
            result as i32
        }
    }
}