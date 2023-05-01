// https://leetcode.com/problems/combination-sum/solutions/2058686/rust-dfs-backtracking/
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn dfs(candidates:&Vec<i32>, target:i32, i:usize, cur: &mut Vec<i32>, res:&mut Vec<Vec<i32>>){
            if target == 0 { // Found
                res.push(cur.clone());
                return;
            }  else if i>=candidates.len() || target < 0{
                return;
            }
            let c = candidates[i];
            cur.push(c);
            dfs(candidates, target - c, i, cur, res); // at i, count c
            cur.pop();
            dfs(candidates, target, i + 1, cur, res); // at i + 1, doesn't count c
        }
        
        let mut res = vec![];
        dfs(&candidates, target, 0, &mut vec![], &mut res);
        res
    }
}