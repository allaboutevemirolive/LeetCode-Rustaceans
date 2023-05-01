// https://leetcode.com/problems/insert-interval/solutions/3058999/rust-solution/
#[inline]
fn merge_inter(i1: &Vec<i32>, i2: &Vec<i32>) -> Vec<i32> {
    return  vec![std::cmp::min(i1[0],i2[0]), std::cmp::max(i1[1],i2[1])];
}
#[inline]
fn interval_is_less(i1: &Vec<i32>, i2: &Vec<i32>) -> bool {
    return  i2[1] < i1[0];
}
#[inline]
fn interval_is_larger(i1: &Vec<i32>, i2: &Vec<i32>) -> bool {
    return  i2[0] > i1[1];
}

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, mut new_i: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut merged = false;

        if intervals.len() == 0 {
            result.push(new_i);
            return result;
        }

        for inter in intervals.iter() {
            if interval_is_less(&inter,&new_i) {
                if !merged {
                    result.push(new_i.to_vec());
                    merged = true;
                }
                result.push(inter.to_vec());
                continue;
            }
            if interval_is_larger(&inter,&new_i) {
                result.push(inter.to_vec());
                continue;
            }
            new_i = merge_inter(&inter, &new_i);
        }
        if !merged {
            result.push(new_i.to_vec())
        }
        return result;
    }
}