// https://leetcode.com/problems/distinct-echo-substrings/solutions/563429/rust-hash-solution/
use std::collections::HashSet;
use std::ops::RangeInclusive;

const PRIME: u128 = 29;

#[derive(Debug)]
struct RabinKarp {
    arr: Vec<u128>,
}

impl RabinKarp {
    fn new(data: &str) -> Self {
        let arr = data
            .chars()
            .enumerate()
            .scan(0, |acc, (ind, chr)| {
                let cur = (chr as u128) - ('a' as u128) + 1;
                *acc = *acc + cur * PRIME.pow(ind as u32);
                Some(*acc)
            })
            .collect();

        RabinKarp { arr }
    }

    fn get_hash(&self, range: RangeInclusive<usize>) -> u128 {
        let hash = if *range.start() > 0 {
            self.arr[*range.end()] - self.arr[*range.start() - 1]
        } else {
            self.arr[*range.end()]
        };

        hash * PRIME.pow((self.arr.len() - *range.start()) as u32)
    }

    fn is_same(&self, first: RangeInclusive<usize>, second: RangeInclusive<usize>) -> bool {
        self.get_hash(first) == self.get_hash(second)
    }
}

impl Solution {
    pub fn distinct_echo_substrings(text: String) -> i32 {
        let hash = RabinKarp::new(&text);
        let mut answer = 0;

        for len in (2..=text.len()).step_by(2) {
            let mut substrings: HashSet<u128> = HashSet::new();
            for i in (len - 1)..text.len() {
                let mid = (i + i - len) / 2;
                let left = (i - len + 1)..=mid;
                let right = mid + 1..=i;
                if hash.is_same(left, right) {
                    let slice = (i - len + 1)..=i;
                    substrings.insert(hash.get_hash(slice));
                }
            }
            answer += substrings.len() as i32;
        }

        answer
    }
}