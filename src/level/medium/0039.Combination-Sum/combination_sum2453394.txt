// https://leetcode.com/problems/combination-sum/solutions/2453394/rust-solution-using-dfs/
struct Helper {
    candidates: Vec<i32>,
    target: i32,
    result: Vec<Vec<i32>>
}

impl Helper {
    fn dfs(&mut self, base: &mut Vec<i32>, now:i32, ci:usize) {
        if self.target == now {
            let mut arr = vec![];
            for i in 1..base.len() {
                for _ in 0..base[i] {
                    arr.push(i as i32);
                }
            }
            self.result.push(arr);
        }

        for i in ci..self.candidates.len() {
            let v = self.candidates[i];
            if v + now <= self.target {
                base[v as usize] += 1;
                self.dfs(base, v + now, i);
                base[v as usize] -= 1;
            }
        }
    }
}

impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();
        for i in (0..candidates.len()).rev() {
            if target < candidates[i] {
            candidates.pop();
            }
        }
            
        let c = candidates.clone();
        let mut base = vec![0;target as usize + 1];
        let mut helper = Helper { candidates, target, result: vec![] };
        for i in 0..c.len() {
            let v = c[i];
            if target < v { break }
            base[v as usize] += 1;
            helper.dfs(&mut base, c[i], i);
            base[v as usize] -= 1;
        }       
        helper.result
    }
}