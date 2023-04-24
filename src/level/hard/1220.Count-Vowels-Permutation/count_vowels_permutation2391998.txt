// https://leetcode.com/problems/count-vowels-permutation/solutions/2391998/rust-one-liner-with-explanation/
const MOD: i32 = 1000000007;
const N_LETTERS: usize = 5;
const A: usize = 0;
const E: usize = 1;
const I: usize = 2;
const O: usize = 3;
const U: usize = 4;

impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        let add_mod = |n1: i32, n2: i32| (n1 + n2) % MOD;
        (1..n).fold([1; N_LETTERS], |s, _| {
            [
                add_mod(add_mod(s[E], s[I]), s[U]),
                add_mod(s[A], s[I]),
                add_mod(s[E], s[O]),
                s[I],
                add_mod(s[I], s[O]),
            ]
        }).into_iter().fold(0, |acc, i| add_mod(acc, i))
    }
}