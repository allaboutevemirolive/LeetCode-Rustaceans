// https://leetcode.com/problems/3sum/solutions/1516971/rust-20ms/
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut my_vec: Vec<Vec<i32>> = vec![];
        //let mut my_vec2: Vec<Vec<i32>> = vec![];
        nums.sort();
        if nums.len() < 3{
            return my_vec;
        }
        let mut i = 0;
        loop {
            if nums[i]>=1 {
                break;
            }
			let mut j = i+1;
            let mut k = nums.len();
            loop {
				if j == nums.len()-1 || j>= k{
					break;
				}
                let last = -1*(nums[i]+nums[j]);
				if last < 0 {
					break;
				}
                
                for l in (j+1..k).rev(){
                    if nums[l] == last{
                        //println!("{}", nums[l]);
                        my_vec.push([nums[i], nums[j], nums[l]].to_vec());
                        k = l;
                        break;
                    }
                    else if nums[l] < last{
                        break;
                    }
                    else{
                        k = l;
                    }
                } 
                let last = found_last(&nums, j, 1);
                if j != last {
                    j = last;
                    continue;    
                }
                break;
            }
            let last = found_last(&nums, i, 2);
                if i != last {
                    i = last;
                    continue;    
                }
            break;
        }
        my_vec
    }
}

fn found_last(nums: &Vec<i32>, i: usize, j: usize) -> usize{
    for i2 in i+1..nums.len()-j{
        if nums[i] < nums[i2]{
            return i2;
        }
    }
    i
}