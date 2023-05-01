// https://leetcode.com/problems/single-number-ii/solutions/700165/rust-c-pure-bit-operation-approach-linear-time-and-constant-space-with-explanation/
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut a:u32 = 0;
        let mut b:u32 = 0;
        for &val in &nums {
            let x = (val+std::i32::MIN) as u32;
            a ^= b&0xffffffff&(!(x^b)); // a switch when b goes from 1 to 0
            b ^= x; // b swicht when x is '1'
            let c = !(a&b); // get position where a and b both are 1
            a = a&c;
            b = b&c;
        }
		// whatever left is the single number
        (b-2147483648) as i32
    }
}