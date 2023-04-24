// https://leetcode.com/problems/number-of-submatrices-that-sum-to-target/solutions/2298205/rust-slow-and-fast-method/
impl Solution {
	pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
		let m: usize = matrix.len();        
		let n: usize = matrix[0].len();
		let mut pre_sum = vec![vec![0;n+1];m+1];
		for i in 1..=m {
			for j in 1..=n {
				pre_sum[i][j]=-pre_sum[i-1][j-1]+pre_sum[i-1][j]+pre_sum[i][j-1]+matrix[i-1][j-1];
			}
		}
		// println!("{:?}", pre_sum);
		let mut ans: i32 = 0;
		for sr in 1..=m {
			for sc in 1..=n {
				for er in sr..=m {
					for ec in sc..=n {
						if pre_sum[er][ec]-pre_sum[er][sc-1]-pre_sum[sr-1][ec]+pre_sum[sr-1][sc-1]== target {ans+=1;}
					}
				}
			}
		}
		ans
	}
}