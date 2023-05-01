// https://leetcode.com/problems/permutations/solutions/2748736/rust-dealing-with-digits/
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {  
        let mut data = nums.clone();
        let mut ret = vec![nums];
        
        loop {
            Self::next_permutation(&mut data);
            
            let mut same = true;  
            for i in 0 .. data.len() {
                if data[i] != ret[0][i] {  same = false; }
            }
            if same { break }
            ret.push(data.clone());
        }
        
        ret
    }
    
    fn next_permutation(nums: &mut Vec<i32>) {
        let n = nums.len();
        let mut k = n;
        
        for i in (0..n - 1).rev() {
            if nums[i] >= nums[i + 1] { continue }
            k = i;
            break
        }
        
        if k == n {
            nums.reverse();
            return
        }
        
        let mut l = k + 1;
        for i in k + 2..n {
            if nums[i] <= nums[k] || nums[i] >= nums[l] { continue }
            l = i;
        }
        
        nums.swap(k, l);
        nums[k + 1..].sort();
    }
}
