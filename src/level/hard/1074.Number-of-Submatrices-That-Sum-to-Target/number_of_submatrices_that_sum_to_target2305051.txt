// https://leetcode.com/problems/number-of-submatrices-that-sum-to-target/solutions/2305051/simple-rust-solution/
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        
        let num_rows = num_rows as usize;
        let mut result: Vec<Vec<i32>> = Vec::with_capacity(num_rows);
        
        for i in 0..num_rows {
            
            let mut vec = vec![1; i + 1];
            
            for x in 1..i {
                vec[x] = result[i - 1][x - 1] + result[i - 1][x];
            }
            
            result.push(vec);
            
        }
        
        return result;
        
    }
}