// https://leetcode.com/problems/profitable-schemes/solutions/3440774/rust-sort-dp-100-fastest/
use std::collections::HashMap;


impl Solution {
    pub fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut hm = HashMap::new();
        let mut grouped: Vec<_> = group.into_iter().zip(profit).collect();
        grouped.sort_unstable();
        Solution::rec(&grouped, 0, n, min_profit, &mut hm) as i32
    }

    pub fn rec(grouped : &Vec<(i32, i32)>, i: usize, people: i32, profit_left: i32, mem: &mut HashMap<(usize, i32, i32), i64>) -> i64 {
        if i == grouped.len() || grouped[i].0 > people {
            return if profit_left <= 0 {1} else {0};
        }

        if let Some(&x) =  mem.get(&(i, people, 0.max(profit_left))) {
            // println!("Overlap!");
            return x;
        }

        let mut ways = 0;

        ways +=  Solution::rec(grouped, i+1, people, profit_left, mem) ;
        ways %=  1_000_000_007;

        ways += Solution::rec( grouped, i+1, people - grouped[i].0, profit_left - grouped[i].1, mem);
        ways %= 1_000_000_007;

        mem.insert((i, people, 0.max(profit_left) ), ways);

        return ways % 1_000_000_007
    }
}