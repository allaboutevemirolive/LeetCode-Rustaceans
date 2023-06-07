// https://leetcode.com/problems/count-vowels-permutation/solutions/2390369/rust-matrix-multiplication/
const OUT_MOD: usize = 1_000_000_007;

impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        if n == 1 { return 5; }
        let mut old = [ 1 as usize; 5 ];
        let mut buf = [ 0 as usize; 5 ];
        for _ in 1 .. n {
            buf[0] = (old[1] + old[2] + old[4]) % OUT_MOD;
            buf[1] = (old[0] + old[2]) % OUT_MOD;
            buf[2] = (old[1] + old[3]) % OUT_MOD;
            buf[3] = old[2];
            buf[4] = (old[2] + old[3]) % OUT_MOD;
            old.copy_from_slice( &buf );
        }
        let out = old[0] + old[1] + old[2] + old[3] + old[4];
        (out % OUT_MOD) as i32
    }
}

// Time: O(n)
// Space: O(1)