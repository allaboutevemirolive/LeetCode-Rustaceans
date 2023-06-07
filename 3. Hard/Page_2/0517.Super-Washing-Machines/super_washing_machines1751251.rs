// https://leetcode.com/problems/super-washing-machines/solutions/1751251/rust-solution/
impl Solution {
    pub fn find_min_moves(machines: Vec<i32>) -> i32 {
        if machines.len() == 0 {
            return -1;
        }

        let sum: i32 = machines.iter().sum();

        if sum % (machines.len() as i32) != 0 {
            return -1;
        }

        let target = sum / (machines.len() as i32);

        let mut to_right = 0;
        let mut result = 0;

        for machine in machines.iter() {
            to_right = machine + to_right - target;
            result = result.max((machine - target).max(to_right.abs()));
        }

        result
    }
}