// https://leetcode.com/problems/combinations/solutions/697301/rust-dbs-backtracking-8ms-o-n-n-k-k/
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        Self::dfs(n, k, 1, &mut vec![], &mut res);
        res
    }
    
    fn dfs(n: i32, k: i32, start: i32, comb: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if comb.len() as i32 == k {
            res.push(comb.clone());
            return;
        }
        
        for i in start..(n+1) {
            comb.push(i);
            Self::dfs(n, k, i + 1, comb, res);
            comb.pop();
        }
    }
}