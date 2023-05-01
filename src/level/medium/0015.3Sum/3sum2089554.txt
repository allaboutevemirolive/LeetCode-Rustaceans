// https://leetcode.com/problems/3sum/solutions/2089554/rust-brute-force-optimised-solution/
use std::collections::HashSet;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 { return vec![] }
        nums.sort();
        let mut res = vec![];
        let mut set = HashSet::new();
        
        for i in 0..nums.len() { 
            let (mut j, mut k) = (i + 1, nums.len() - 1);
            while j < k { 
                let target = 0;
                let sum = nums[j] + nums[k] + nums[i];
                if sum == target { 
                    set.insert((nums[i], nums[j], nums[k]));
                    j +=1;
                    k -=1;
                } else if sum < target { 
                    j +=1;
                } else { 
                    k -=1
                }
            }
        }
            
        for i in set.iter() { 
            res.push(vec![i.0, i.1, i.2])
        }
        
        res
    }
}