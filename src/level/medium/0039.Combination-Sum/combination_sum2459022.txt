// https://leetcode.com/problems/combination-sum/solutions/2459022/rust-solution-with-backtracking/
// backtracking

fn backtrack(candidates: &Vec<i32>, res: &mut Vec<Vec<i32>>, tmp_list: &mut Vec<i32>, remain:i32, start:i32) {
    if remain < 0 {
        return;
    } else if remain == 0 {
        res.push(tmp_list.to_vec());
        return;
    } else {
        let mut i = start;
        let len = candidates.len() as i32;
        while i < len {
            tmp_list.push(candidates[i as usize]);
            // repeat consume i, that is next start=i
            backtrack(candidates, res, tmp_list, remain-candidates[i as usize], i); 
            // unmake
            tmp_list.pop();
            i += 1;
        }
    }
}

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res:Vec<Vec<i32>> = Vec::new();
        let mut tmp = Vec::new();
        backtrack(&candidates, &mut res, &mut tmp, target, 0);
        return res;
    }
}