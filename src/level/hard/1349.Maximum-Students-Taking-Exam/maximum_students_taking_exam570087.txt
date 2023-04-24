// https://leetcode.com/problems/maximum-students-taking-exam/solutions/570087/rust-with-dfs-and-memo/
use std::collections::HashMap;

fn dfs(seats: Vec<Vec<char>>, mut memo: &mut HashMap<String, i32>) -> i32 {
    let key = format!("{:?}", seats);
    if memo.contains_key(&key) { return memo[&key]; }
    let mut res = 0;
    
    if !seats.iter().all(|r| r.iter().all(|x| *x == '#')) {
        res = res.max(dfs(take_first_available(&seats), &mut memo) + 1);
        res = res.max(dfs(not_take_first_available(&seats), &mut memo));
    }
    
    memo.insert(key, res);
    res
}

fn take_first_available(seats: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let m = seats.len();
    let n = seats[0].len();
    let mut res = vec![vec!['-'; n]; m];
    let mut any_marked = false;
    for i in 0..m {
        for j in 0..n {
            if !any_marked && seats[i][j] == '.' {
                res[i][j] = '#';
                if j > 0 { res[i][j - 1] = '#' }
                if j < n - 1 { res[i][j + 1] = '#' }
                if i < m - 1 && j > 0 { res[i + 1][j - 1] = '#' }
                if i < m - 1 && j < n - 1 { res[i + 1][j + 1] = '#' }
                any_marked = true;
            } else if res[i][j] == '-' {
                res[i][j] = seats[i][j];   
            }
        }
    }
    res
}

fn not_take_first_available(seats: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let m = seats.len();
    let n = seats[0].len();
    let mut res = vec![vec!['-'; n]; m];
    let mut any_marked = false;
    for i in 0..m {
        for j in 0..n {
            if !any_marked && seats[i][j] == '.' {
                res[i][j] = '#';
                any_marked = true;
            } else if res[i][j] == '-' {
                res[i][j] = seats[i][j];   
            }
        }
    }
    res
}

impl Solution {
    pub fn max_students(seats: Vec<Vec<char>>) -> i32 {
        let mut memo = HashMap::new();
        dfs(seats, &mut memo)
    }
}