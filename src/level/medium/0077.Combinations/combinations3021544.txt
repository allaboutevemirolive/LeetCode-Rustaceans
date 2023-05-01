// https://leetcode.com/problems/combinations/solutions/3021544/rust-backtracking-using-recursion/
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        fn backtrack(next: usize, so_far: &mut Vec<i32>, combinations: &mut Vec<Vec<i32>>, n: usize, k: usize) {
            if so_far.len() == k {
                combinations.push(so_far.clone());
                return;
            }
            for i in next..=n {
                so_far.push(i as i32);
                backtrack(i+1, so_far, combinations, n, k);
                so_far.pop();
            }
        }

        let mut so_far : Vec<i32> = Vec::new();
        let mut combinations : Vec<Vec<i32>> = Vec::new();
        backtrack(1, &mut so_far, &mut combinations, n as usize, k as usize);
        combinations
    }
}