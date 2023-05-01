// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/solutions/2071286/rust-solutions/
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
         use std::collections::HashMap;
    let mut result:Vec<i32> = Vec::new();
    let mut hash: HashMap<i32, i32> = HashMap::new();
    for (i, num) in numbers.iter().enumerate(){
        let res=hash.get(num).unwrap_or(&-1);
        if *res == -1 {
            hash.insert(target-num, i as i32);
        }
        else {
            result.push(*res+1);
            result.push(i as i32+1);
            break;
        }
    }

    result
        
    }
	
	using two pointers  is the optimal solution  in my openion 
	```
	impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut pointer_1=0;
    let mut pointer_2=numbers.len()-1;
    
    
    loop{
        if numbers[pointer_2]+numbers[pointer_1]==target{
            
            
            break;

        }
        else if numbers[pointer_2]+numbers[pointer_1]>target{
            pointer_2-=1;
        }
        else if numbers[pointer_2]+numbers[pointer_1]<target{
            pointer_1+=1;
        }
        
        else if pointer_2==pointer_1{
            println!("404 not found");

            return vec![];

                }

    if numbers[pointer_2]+numbers[pointer_1]==target{
        let ve:Vec<i32>=vec![(pointer_1+1) as i32,(pointer_2+1) as i32];
        return ve

    }
    

    }

    return vec![pointer_1 as i32+1, pointer_2 as i32+1];
        
    }
}
}


useing pointers and  pattern matching

i dont know why but this tacks slightly more thime then if else controle flow but still faster then hashmap

``
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
     let mut pointer_1=0;
    let mut pointer_2=numbers.len()-1;
    
    
    loop{
        match numbers[pointer_2]+numbers[pointer_1]{
            x if x==target => {
                return vec![pointer_1 as i32+1, pointer_2 as i32+1];
            },
            x if x<target => {
                pointer_1+=1;
            },
            x if x>target => {
                pointer_2-=1;
            }
            _=>{
            }
        }
    }
    }
}