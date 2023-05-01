// https://leetcode.com/problems/single-number-ii/solutions/1352030/rust-solution/
    pub fn single_number(nums: Vec<i32>) -> i32 {
        for el in &nums{
        let mut count=0;
        for il in &nums{
            if il==el{
                count+=1
            }
        }
    if count==1{
        return *el
        }
    else{
        continue;
    }
    }
    return 0
}
}```