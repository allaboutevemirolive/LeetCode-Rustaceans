// https://leetcode.com/problems/combinations/solutions/3181112/rust-3ms/
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut cnts  = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20];

        loop {
            res.push((&cnts[0..k as usize]).to_vec().clone());
            for i in (-1..k).rev() {
                if i == -1 { 
                    return res;
                }
                cnts[i as usize] += 1;
                if cnts[i as usize] > n - (k-1-i) {
                    continue;                   
                }
                for j in (i+1)..k {
                    cnts[j as usize] = cnts[(j-1) as usize]+1;
                }
                break;
            }
        }
    }
}