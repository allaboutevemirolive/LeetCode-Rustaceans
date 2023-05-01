// https://leetcode.com/problems/factorial-trailing-zeroes/solutions/1113228/rust-solution/
impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut counter_2 = 0;
        let mut counter_5 = 0;
        for i in 1..n+1 {
            let mut x = i;
            let mut y = i;
            while x % 2 == 0 {
                counter_2 += 1;
                x = x/2
            }
            while y % 5 == 0 {
                counter_5 += 1;
                y = y/5
            }
        }
        std::cmp::min(counter_2, counter_5)
    }
}