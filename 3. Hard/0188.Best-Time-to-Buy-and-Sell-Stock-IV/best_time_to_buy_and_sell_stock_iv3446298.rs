// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iv/solutions/3446298/rust-2-approaches/
impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        fn top_down(k: i32, prices: Vec<i32>) -> i32 {
            fn rec(
                dp: &mut Vec<Vec<Vec<i32>>>,
                prices: &Vec<i32>,
                i: usize,
                hold: usize,
                remain: usize,
            ) -> i32 {
                if i == prices.len() || remain == 0 {
                    return 0;
                }
                if dp[i][hold][remain] > -1 {
                    return dp[i][hold][remain];
                }
                let mut ans = rec(dp, prices, i + 1, hold, remain);
                if hold == 1 {
                    ans = ans.max(prices[i] + rec(dp, prices, i + 1, 0, remain - 1));
                } else {
                    ans = ans.max(-prices[i] + rec(dp, prices, i + 1, 1, remain));
                }
                dp[i][hold][remain] = ans;
                dp[i][hold][remain]
            }
            let mut dp = vec![vec![vec![-1; k as usize + 1]; 2]; prices.len()];
            rec(&mut dp, &prices, 0, 0, k as usize)
        }
        fn bottom_up(k: i32, prices: Vec<i32>) -> i32 {
            let mut dp = vec![vec![vec![0; k as usize + 1]; 2]; prices.len() + 1];
            for i in (0..prices.len()).rev() {
                for remain in 1..=k as usize {
                    for holding in 0..2 {
                        let mut ans = dp[i + 1][holding][remain];
                        if holding == 1 {
                            ans = ans.max(prices[i] + dp[i + 1][0][remain - 1]);
                        } else {
                            ans = ans.max(-prices[i] + dp[i + 1][1][remain]);
                        }
                        dp[i][holding][remain] = ans;
                    }
                }
            }
            dp[0][0][k as usize]
        }
        bottom_up(k, prices)    
    }
}