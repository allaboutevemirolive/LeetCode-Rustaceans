// https://leetcode.com/problems/3sum/solutions/1667324/rust-pointers/
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        let mut res: Vec<Vec<i32>> = vec![];
        nums.sort();
        
        for i in 0..nums.len() { 
            if i > 0 && nums[i]==nums[i-1] { 
                continue
            }
            let (mut l, mut r) =  (i+1, nums.len() -1);
            while l < r { 
                let LEFT = nums[l];
                let PREV = nums[l-1];
                let RIGHT = nums[r];
                let NUM = nums[i];
                
                //  If the prev num, is the same as curr num, move forward..
                if l > i + 1 && LEFT == PREV { 
                    l +=1;
                    continue;
                }
                
                if r < nums.len() - 1 && RIGHT == nums[r+1] {
                    r -=1;
                    continue;
                }
                
                let total = LEFT + RIGHT + NUM;
                match total { 
                    total if total == 0 => {
                        res.push([NUM, LEFT, RIGHT].to_vec());
                        l +=1;
                    }
                    total if total < 0 => {l +=1}
                    _ => {r -=1}
                }
            }
        }
        res
    }
}