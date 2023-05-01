// https://leetcode.com/problems/combination-sum/solutions/3158456/rust-recursive-solution/
impl Solution {
    pub fn comn(candidates : & Vec<i32>,target : i32,ans : &mut Vec<Vec<i32>>,temp : &mut Vec<i32>,mut index : usize){
        if target == 0 {
            ans.push(temp.to_vec());
            return;
        }

        if index == candidates.len() {
            return;
        }

        while index < candidates.len() {
            let t : usize = (target/candidates[index]) as usize;
            println!("{} {} {}",t,candidates[index],target);
            for y in 0..t {
                for x in 0..y+1 {
                    temp.push(candidates[index]);
                }
                println!("{:?}",temp);
                Self::comn(candidates, target - (candidates[index] * (y+1) as i32),ans,temp,index+1);
                for x in 0..y+1 {
                    temp.pop();
                }
            }
            index += 1;
        }
    }

    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut ans : Vec<Vec<i32>> = vec![];
        let mut temp : Vec<i32> = vec![];
        Self::comn(& candidates, target, &mut ans, &mut temp, 0 );

        ans
    }
}