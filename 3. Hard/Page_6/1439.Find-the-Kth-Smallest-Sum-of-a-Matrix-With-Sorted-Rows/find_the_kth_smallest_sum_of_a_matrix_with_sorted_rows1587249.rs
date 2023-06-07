// https://leetcode.com/problems/find-the-kth-smallest-sum-of-a-matrix-with-sorted-rows/solutions/1587249/rust-backtracking-8ms/
fn push(x: i32, smallest_list: &mut Vec<i32>, k: usize) -> bool {
    if smallest_list.is_empty() {
        smallest_list.push(x);
        true
    } else {
        if x < smallest_list[smallest_list.len() - 1] {
            let mut temp: Vec<i32> = Vec::with_capacity(smallest_list.len());
            temp.push(smallest_list.pop().unwrap());
            while !smallest_list.is_empty() 
                && smallest_list[smallest_list.len() - 1] > x {
                temp.push(smallest_list.pop().unwrap());
            }
            smallest_list.push(x);
            while !temp.is_empty() && smallest_list.len() < k {
                smallest_list.push(temp.pop().unwrap());
            }
            true
        } else {
            if smallest_list.len() < k {
                smallest_list.push(x);
                true
            } else {
                false
            }
        }
    }
}

fn build(mat: &[Vec<i32>], mut cur_sum: i32, first_k: &mut Vec<i32>, k: usize) -> bool {
    if !mat.is_empty() {
        let mut seen_success: bool = false;
        for i in 0..mat[0].len() {
            cur_sum += mat[0][i];
            if first_k.len() < k || cur_sum < first_k[first_k.len() - 1] {
                if build(&mat[1..], cur_sum, first_k, k) {
                    seen_success = true;
                } else {
                    break;
                }
            }
            cur_sum -= mat[0][i];
        }
        seen_success
    } else {
        push(cur_sum, first_k, k)
    }
}

impl Solution {
    pub fn kth_smallest(mat: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut first_k: Vec<i32> = Vec::with_capacity(k as usize + 1);
        let mut cur_sum: i32 = 0;
        
        build(&mat, 0, &mut first_k, k as usize);
        
        match first_k.pop() {
            Some(value) => value,
            None => 0
        }
    }
}