// https://leetcode.com/problems/triples-with-bitwise-and-equal-to-zero/solutions/391361/a-simple-rust-solution/
impl Solution {
    pub fn count_triplets(a: Vec<i32>) -> i32 {
        let mut result = 0;
        for i in &a {
            for j in &a {
                for k in &a {
                    if i & j & k == 0 {
                        result += 1;
                    }
                }
            }
        }
        result
    }
}