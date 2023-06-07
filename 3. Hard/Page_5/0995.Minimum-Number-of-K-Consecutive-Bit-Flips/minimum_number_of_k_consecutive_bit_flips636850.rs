// https://leetcode.com/problems/minimum-number-of-k-consecutive-bit-flips/solutions/636850/rust-gready-deque-solution-o-n-time-and-o-k-space/
use std::collections::VecDeque;

impl Solution {
    pub fn min_k_bit_flips(arr: Vec<i32>, k: i32) -> i32 {
        let n = arr.len();
        let k = k as usize;
        let mut queue: VecDeque<usize> = VecDeque::new();

        let mut answer = 0;
        for (idx, &current_coin) in arr.iter().enumerate() {
            while let Some(&flip_start) = queue.front() {
                if flip_start + k <= idx {
                    queue.pop_front();
                } else {
                    break;
                }
            }

            let flip_count = queue.len();
            if (flip_count + current_coin as usize) % 2 == 0 {
                if idx + k > n {
                    return -1;
                }

                answer += 1;
                queue.push_back(idx);
            }
        }

        answer
    }
}