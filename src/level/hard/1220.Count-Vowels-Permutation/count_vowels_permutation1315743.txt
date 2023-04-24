// https://leetcode.com/problems/count-vowels-permutation/solutions/1315743/rust-solution/
const MOD: u32 = 1_000_000_007;

impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        ((1..n)
            .fold([1; 5], |v, _| {
                [
                    (v[1] + v[2] + v[4]) % MOD,
                    (v[0] + v[2]) % MOD,
                    (v[1] + v[3]) % MOD,
                    (v[2]) % MOD,
                    (v[2] + v[3]) % MOD,
                ]
            })
            .iter()
            .sum::<u32>()
            % MOD) as i32
    }
}