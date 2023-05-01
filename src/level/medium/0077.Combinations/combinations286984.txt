// https://leetcode.com/problems/combinations/solutions/286984/backtracking-vs-simple-recursive-rust/
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut all_solutions:Vec<Vec<i32>> = vec![];
        Self::combine_help(n, k, 1,  &mut Vec::new(),  &mut all_solutions );
        all_solutions
    }
    
    fn combine_help (n:i32, k:i32, index: i32, one_solution:&mut Vec<i32>, all_solutions:&mut Vec<Vec<i32>>) {
        if k == 0 {
            all_solutions.push(one_solution.to_vec());
            return
        }
        
        for i in index ..= n - k + 1{
            one_solution.push(i);
            Self::combine_help(n, k-1, i + 1 , one_solution, all_solutions);
            one_solution.pop();
        }
    }
}