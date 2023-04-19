// https://leetcode.com/problems/max-sum-of-rectangle-no-larger-than-k/solutions/248524/rust-solution/
impl Solution {
   
    pub fn binary_search(matrix: &Vec<i32>, target: i32) -> usize {
        let mut left = 0usize;
        let mut right = matrix.len() ;
        while left < right {
            let mid = left + (right - left) / 2;
            if matrix[mid] == target {
                return mid;
            } else if matrix[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        return left;
    }
    
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        if matrix.is_empty() || matrix[0].is_empty() {
            return 0;
        } else {
            let mut sumMax = std::i32::MIN;
            let row = matrix.len();
            let col = matrix[0].len();
            for left in 0..col {
                let mut sum: Vec<i32> = vec![0; row];
                for right in left..col {
                    for idx in 0..row {
                        sum[idx] += matrix[idx][right];
                    }
                    let mut curSum = 0;
                    let mut temp: Vec<i32> = vec![0];
                    for idx in 0..row {
                        curSum += sum[idx];
                        let ix = Solution::binary_search(&temp, curSum - k);
                        if (ix < temp.len()) {
                            sumMax = std::cmp::max(curSum - temp[ix], sumMax);
                        }
                        let ix = Solution::binary_search(&temp, curSum);
                        temp.insert(ix, curSum);
                    }
                }
            }
            return sumMax;
        }
    }
}