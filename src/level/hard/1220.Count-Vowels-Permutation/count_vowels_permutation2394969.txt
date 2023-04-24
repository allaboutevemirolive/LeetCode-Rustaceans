// https://leetcode.com/problems/count-vowels-permutation/solutions/2394969/rust-dp/
use std::collections::HashMap;
impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        let mut v = [[0; 5]; 2];         
        // a e i o u
        
        let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
        map.insert(0, vec![1, 2, 4]);
        map.insert(1, vec![0, 2]);
        map.insert(2, vec![1, 3]);
        map.insert(3, vec![2]);
        map.insert(4, vec![2, 3]);
        
        v[0] = [1,1,1,1,1];
        
        for i in 1..n as usize {
            for j in 0..5 {
                let mut sum = 0;

                for &k in map.get(&j).unwrap() {
                    sum = (sum + v[(i-1)%2][k]) % 1_000_000_007;
                }


                v[i%2][j] = sum;
            }
        }

        v[((n-1)%2) as usize].iter().fold(0, |acc, s| (acc + s) % 1_000_000_007)
    }

}