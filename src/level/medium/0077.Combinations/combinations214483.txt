// https://leetcode.com/problems/combinations/solutions/214483/rust-solution/
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        if k == 0 {
            return vec![vec![]];
        }
        let mut rtn = vec![];
        for i in k..=n {
            for mut pre in Self::combine(i-1, k-1) {
                pre.push(i);
                rtn.push(pre);
            }
        }
        return rtn;
    }
}