// https://leetcode.com/problems/bitwise-and-of-numbers-range/solutions/2959033/rust/
/* 
3: 0011
4: 0100
5: 0101
6: 0110
&: 0100

*/
impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        let mut tmp = right-left;
        let mut mask = 0;
        while tmp>0 {
            tmp>>=1;
            mask = mask<<1 | 1;
        }

        right & left & (!mask)
    }
}