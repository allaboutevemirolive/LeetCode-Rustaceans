// https://leetcode.com/problems/can-make-arithmetic-progression-from-sequence/solutions/3602627/rust-benchmark-o-n-vs-o-nlogn/
impl Solution {
    pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
        let n = arr.len() as i32;
        let min = *arr.iter().min().unwrap_or(&0);
        let max = *arr.iter().max().unwrap_or(&0);
        if (max-min)%(n-1) != 0 {
            return false;
        }
        let d = (max-min)/(n-1);
        if d == 0 {
            return true;
        }
        let mut vis = vec![false;n as usize];
        arr.iter()
            .map(|x| ((x-min)/d) as usize)
            .for_each(|x| vis[x] = true);
        vis.iter().all(|&x| x)
    }
}