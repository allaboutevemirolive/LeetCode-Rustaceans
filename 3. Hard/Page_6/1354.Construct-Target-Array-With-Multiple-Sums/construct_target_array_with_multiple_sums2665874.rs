// https://leetcode.com/problems/construct-target-array-with-multiple-sums/solutions/2665874/rust-6ms-verbose-though/
impl Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        if target.len() == 1 { return target[0] == 1; }
        let mut new_target = Vec::new();
        for &n in target.iter() {
            new_target.push(n as i64);
        }
        let mut target = new_target;
        let mut sum_value: i64 = target.iter().sum();
        loop {
            let (mut max_value, mut sub_value, mut max_idx) = (0, -1, 0);
            for (i, &n) in target.iter().enumerate() {
                if n > max_value {
                    sub_value = max_value;
                    max_value = n;
                    max_idx = i;
                } else if n > sub_value {
                    sub_value = n;
                }
            }
            if max_value < 1 { return false; }
            if max_value == 1 && sum_value == target.len() as i64 { return true; }
            let diff = sum_value - max_value;
            if max_value - diff < 1 { return false; }
            let multiple = (max_value - sub_value + diff - 1) / diff;
            let sub = multiple * diff;
            sum_value -= sub;
            target[max_idx] -= sub;
        }
    }
}