// https://leetcode.com/problems/count-vowels-permutation/solutions/1316151/rust-o-log-n-matrix-exponentiation-0ms/
use std::ops::Mul;

const MOD: i64 = 1_000_000_007;

#[derive(Clone, Debug)]
struct SqMatrix {
    n: usize,
    matrix: Vec<i64>,
}

fn dot_product<'a, 'b, I, I2>(lhs: I, rhs: I2) -> i64
where I: Iterator<Item=&'a i64>,
I2: Iterator<Item=&'b i64> {
    lhs.zip(rhs).map(|(&l,&r)| (l*r) % MOD).fold(0, |sum, v| (sum + v) % MOD)
}

impl Mul for SqMatrix {
    type Output = SqMatrix;
    
    fn mul(self, rhs: SqMatrix) -> SqMatrix {
        let n = self.n;
        let mut r_vec = vec![0; n*n];
        for c in 0..n {
            let col_vec = rhs.matrix[c..].iter().step_by(n).map(|&v|v).collect::<Vec<_>>();
            for r in 0..n {
                let mut left_iter = self.matrix[(r*n)..((r+1)*n)].iter();
                let mut right_iter = col_vec.iter();
                let v = dot_product(left_iter, right_iter);
                r_vec[r*n + c] = v;
            }
        }
        SqMatrix::new(r_vec, n)
    }   
}

impl SqMatrix {
    pub fn new(matrix: Vec<i64>, n: usize) -> Self {
        Self {
            n: n,
            matrix: matrix,
        }
    }
}


impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        let mut pow_matrix = SqMatrix::new(
            vec![
                0, 1, 0, 0, 0,
                1, 0, 1, 0, 0,
                1, 1, 0, 1, 1,
                0, 0, 1, 0, 1,
                1, 0, 0, 0, 0
            ], 5
        );
        let mut answer_mat = SqMatrix::new(
            vec![
                1, 0, 0, 0, 0,
                0, 1, 0, 0, 0,
                0, 0, 1, 0, 0,
                0, 0, 0, 1, 0,
                0, 0, 0, 0, 1
            ], 5
        );
        let n = n - 1;
        let mut b = n;
        while b > 0 {
            if b % 2 == 1 {
                answer_mat = answer_mat.clone() * pow_matrix.clone();
            }
            b >>= 1;
            pow_matrix = pow_matrix.clone() * pow_matrix;
        }
        answer_mat.matrix.iter().copied().fold(0, |sum, v| (sum + v) % MOD) as i32
    }
}