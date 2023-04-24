// https://leetcode.com/problems/count-vowels-permutation/solutions/1316449/stupid-simple-rust-solution/
impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        
        let mut a_count = 1;
        let mut e_count = 1;
        let mut i_count = 1;
        let mut o_count = 1;
        let mut u_count = 1;
        let MOD = 1000000007;
        for i in 1..n {
            let new_a_count = e_count;
            let new_e_count = (a_count + i_count) % MOD;
            let new_i_count = ((((a_count + e_count) % MOD) + o_count) % MOD + u_count) % MOD;
            let new_o_count = (i_count + u_count) % MOD;
            let new_u_count = a_count;
            a_count = new_a_count;
            e_count = new_e_count;
            i_count = new_i_count;
            o_count = new_o_count;
            u_count = new_u_count;            
        }
        
        ((((((a_count + e_count) % MOD) + i_count) % MOD + o_count) % MOD) + u_count) % MOD
        
        
    }
}