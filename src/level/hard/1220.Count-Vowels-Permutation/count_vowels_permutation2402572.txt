// https://leetcode.com/problems/count-vowels-permutation/solutions/2402572/rust-simple-fold-and-bonus-closure/
impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        let modulo: i64 = 10_i64.pow(9) + 7;
        ((2..=n)
            .fold([1i64; 5], |acc, _| {
                [
                    acc[1],
                    acc[0] + acc[2] % modulo,
                    acc[0] + acc[1] + acc[3] + acc[4] % modulo,
                    acc[2] + acc[4] % modulo,
                    acc[0],
                ]
            })
            .iter()
            .sum::<i64>()
            % modulo) as i32
    }
}