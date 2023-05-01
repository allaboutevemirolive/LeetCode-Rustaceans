// https://leetcode.com/problems/combinations/solutions/1129770/rust-backtracking-solution/
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        Self::dfs(n, k, &mut vec![], 1, &mut res);
        res
    }
    
    fn dfs(n: i32, k: i32, acc: &mut Vec<i32>, v: i32, res: &mut Vec<Vec<i32>>) {
        if acc.len() == k as usize {
            res.push(acc.clone());
            return;
        }
        
        for i in v..=n {
            acc.push(i);
            Self::dfs(n, k, acc, i + 1, res);
            acc.pop();
        }
    }
}