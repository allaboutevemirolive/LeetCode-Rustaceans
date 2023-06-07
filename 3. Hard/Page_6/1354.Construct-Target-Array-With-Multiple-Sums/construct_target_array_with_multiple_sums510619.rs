// https://leetcode.com/problems/construct-target-array-with-multiple-sums/solutions/510619/rust-4ms-very-fast-solution/
impl Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        let mut new_tar:Vec<i64> = Vec::new();
        for &i in target.iter() {
            new_tar.push(i as i64);
        }
        let mut sum: i64 = new_tar.iter().sum();
        let mut max_ind:usize = 0;
        loop {
            for i in 0..new_tar.len() {
                if new_tar[i] <= 0 {
                    return false;
                }
                if new_tar[i] > new_tar[max_ind] {
                    max_ind = i;
                }
            }
        
            if new_tar[max_ind] == 1 {
                break true;
            }
            let old_val = new_tar[max_ind] - (sum - new_tar[max_ind]);
            sum = (sum - new_tar[max_ind]) + old_val;
            new_tar[max_ind] = old_val;
        }
    }
}