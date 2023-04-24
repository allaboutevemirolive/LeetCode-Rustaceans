// https://leetcode.com/problems/maximum-performance-of-a-team/solutions/2559883/rust-the-best-and-brightest/
use std::cmp::Reverse;
use std::collections::BinaryHeap;

// speed, eff
#[derive(Eq,PartialEq,PartialOrd,Ord)]
struct Engineer(i32, i32);

const ANSWER_MODULUS: u64 = 1_000_000_007;

impl Solution {
    pub fn max_performance(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
        let mut engs: Vec<Engineer> = speed.into_iter()
            .zip(efficiency.into_iter())
            .map(|(s,e)| Engineer(s, e))
            .collect();
        // ORDER BY eff DESC, speed DESC
        engs.sort_unstable_by( |e1,e2|
            if e1.1 != e2.1 {  // faster than .then
                e1.1.cmp(&e2.1).reverse()
            } else {
                e1.0.cmp(&e2.0).reverse()
            }
        );
        let engs = engs;
        let K = k as usize;
        let mut fastest = BinaryHeap::new();
        let mut fastest_sum: u64 = 0;
        let mut best = 0_u64;
        for eng in engs {
            let Engineer(speed, eff) = eng;
            fastest.push(Reverse(eng));
            fastest_sum += speed as u64;
            if fastest.len() > K {
                let Reverse(too_slow) = fastest.pop().unwrap();
                fastest_sum -= too_slow.0 as u64;
            }
            // We now have the fastest `K` engineers with efficiency `>= eff`.
            let prod: u64 = fastest_sum * (eff as u64);
            best = best.max(prod);
        }
        (best % ANSWER_MODULUS) as i32
    }
}