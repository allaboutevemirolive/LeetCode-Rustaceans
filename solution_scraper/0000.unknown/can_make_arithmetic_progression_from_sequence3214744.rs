// https://leetcode.com/problems/can-make-arithmetic-progression-from-sequence/solutions/3214744/rust-fast-simple-solution/
impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        arr.sort();
        let mut counter = 0;
        if arr.windows(2).any(|vec| vec[1] - vec[0] != arr[1] - arr[0]){
            counter -=1;
        };
        println!("{counter}");
        if  arr.windows(2).rev().any(|vec| vec[1] - vec[0] != arr[arr.len()-2] - arr[arr.len()-1]) {
            counter -=1;
        };
        match counter {
            0 | -1  => return true,
            _ => return false
        }
    }
}