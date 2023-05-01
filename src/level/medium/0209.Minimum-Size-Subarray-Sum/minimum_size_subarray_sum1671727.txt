// https://leetcode.com/problems/minimum-size-subarray-sum/solutions/1671727/rust-0ms-2-5mb/
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let (mut res, mut val_sum, mut l) = (
            std::usize::MAX, 0, 0);
        
        (0..nums.len())
        .fold(0, |(mut val_sum), r| {    
            val_sum += nums[r];
            while val_sum >= target { 
                res = res.min(r - l + 1);
                //  if the sum > target, we need to substract curr
                //  num to our total to avoid overflowing from our target 
                //  Move pointer forward 
                val_sum -= nums[l];
                l +=1;
            } 
            val_sum
        });
        if res == usize::MAX { 
            0 
        } else { 
            res as i32
        }
    }
}