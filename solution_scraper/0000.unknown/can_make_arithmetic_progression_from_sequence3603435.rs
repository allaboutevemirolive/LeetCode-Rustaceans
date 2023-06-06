// https://leetcode.com/problems/can-make-arithmetic-progression-from-sequence/solutions/3603435/rust-solution-simple-mathematics/
impl Solution {
    pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
        let mut data : Vec<i32> = Vec::new();

        for x in arr {
            data.push(x);
        }

        data.sort();

        let diff = data[1]-data[0];
        let mut i = 2;

        while i<data.len() {
            if(data[i]-data[i-1] != diff){
                return false;
            }

            i = i+1;
        }

        return true;
    }
}