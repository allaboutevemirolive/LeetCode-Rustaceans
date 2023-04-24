// https://leetcode.com/problems/reducing-dishes/solutions/3356741/rust-greedy-sorting-100/
impl Solution {
    pub fn max_satisfaction(mut satisfaction: Vec<i32>) -> i32 {
        let len = satisfaction.len();
        //sort the array in ascending order
        satisfaction.sort();

        // calculate the sum for the last i elements of the sorted array
        let mut sum = 0;
        for i in 0..len {
            let mut curr_sum = 0;
            let mut time = 1;
            for j in len-i-1..len {
                curr_sum += time * satisfaction[j];
                time += 1;
            }

            // check if the calculated sum of last i elements is bigger than sum of i-1 elements
            if curr_sum > sum {
                sum = curr_sum;
            } else {
                break;
            }
            curr_sum = 0;
        }

        sum
    }
}