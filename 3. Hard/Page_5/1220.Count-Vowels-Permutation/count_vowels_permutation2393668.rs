// https://leetcode.com/problems/count-vowels-permutation/solutions/2393668/rust-solution/
const MOD : usize = 1000000007;
type Mat = [[usize;5];5];

impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        let mut A = [[0,1,1,0,1],[1,0,1,0,0],[0,1,0,1,0],[0,0,1,0,0],[0,0,1,1,0]];
        
        fn mul(A: &Mat, B: &Mat) -> Mat {
            let mut C = [[0;5];5];
            for k in 0..5 {
                for i in 0..5 {
                    for j in 0..5 {C[i][j]+=A[i][k]*B[k][j] % MOD;}
                }
            }
            C
        };
        
        fn pw(mut n: i32, A: &mut Mat) -> Mat {
            let I = [[1,0,0,0,0],[0,1,0,0,0],[0,0,1,0,0],[0,0,0,1,0],[0,0,0,0,1]];
            let mut R = if n%2==0 {I} else {A.clone()};
            while n/2>0 {
                n/=2;
                *A=mul(A,A);
                if n%2==1 {R=mul(&R,A);}
            }
            R
        }
        (pw(n-1, &mut A).iter().flatten().sum::<usize>() % MOD) as i32
    }
}