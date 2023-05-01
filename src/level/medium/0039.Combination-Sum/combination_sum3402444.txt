// https://leetcode.com/problems/combination-sum/solutions/3402444/dfs-in-rust/
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        let mut can: Vec<i32> = vec![];
        Self::helper(&candidates, &mut res, &mut can, target, 0);
        return res;

    }

    fn helper(
        candidates: &Vec<i32>,
        mut res: &mut Vec<Vec<i32>>,
        mut can: &mut Vec<i32>,
        target: i32,
        idx: usize
    ) {
        if target < 0 {
            return;
        }

        if target == 0 {
            res.push(can.clone());
            return;
        }

        for i in idx..candidates.len() {
            // let val = candidates[i].clone();
            can.push(candidates[i]);
            Self::helper(candidates, res, can, target - candidates[i], i);
            can.pop();
        }
    }
}