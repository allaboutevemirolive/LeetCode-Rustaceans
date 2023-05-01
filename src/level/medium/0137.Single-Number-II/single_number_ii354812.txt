// https://leetcode.com/problems/single-number-ii/solutions/354812/rust-100-speed-and-memory/
use std::collections::HashMap;
    
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut digits = HashMap::new();
        for num in nums {
            match digits.get(&num) {
                Some(n) => {
                    match n {
                        1 => digits.insert(num, 2),
                        2 => digits.remove(&num),
                        &_ => None,
                    }
                },
                None => digits.insert(num, 1)
            };
        }
        let mut output = 0;
        for (digit, boolean) in digits {
            output = digit;
        }
        output
    }
}