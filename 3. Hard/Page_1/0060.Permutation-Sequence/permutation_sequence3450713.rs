// https://leetcode.com/problems/permutation-sequence/solutions/3450713/rust-simple-solution-beats-100/
const DIGIT_SPAN: [usize; 10] = [0, 1, 1, 2, 6, 24, 120, 720, 5040, 40320];

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut digits = vec![b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9'];
        let mut res: Vec<u8> = Vec::with_capacity(n as usize);
        let mut k = (k - 1) as usize;

        for i in (1..=n as usize).rev() {
            let digit_span = DIGIT_SPAN[i];
            let digits_skip = k / digit_span;
            res.push(digits.remove(digits_skip));
            k -= digits_skip * digit_span;
        }

        unsafe {String::from_utf8_unchecked(res)}
    }
}