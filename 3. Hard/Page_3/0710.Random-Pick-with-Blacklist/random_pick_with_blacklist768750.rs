// https://leetcode.com/problems/random-pick-with-blacklist/solutions/768750/rust-translated/
use rand::prelude::*;
use std::collections::HashMap;

struct Solution {
    rng: ThreadRng,
    map: HashMap<i32, i32>,
    m: i32,
}

impl Solution {
    fn new(mut n: i32, blacklist: Vec<i32>) -> Self {
        let mut map = HashMap::<i32, i32>::new();
        let rng = Default::default();
        for &b in &blacklist {
            map.insert(b, -1);
        }
        let m = n - map.len() as i32;
        for b in blacklist {
            if b < m {
                while map.contains_key(&(n - 1)) {
                    n -= 1;
                }
                map.insert(b, n - 1);
                n -= 1;
            }
        }
        Solution { rng, map, m }
    }

    fn pick(&mut self) -> i32 {
        let p = self.rng.gen_range(0, self.m);
        if self.map.contains_key(&p) {
            *self.map.get(&p).unwrap()
        } else {
            p
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_pick() {
        let mut s = Solution::new(10, vec![2, 7, 9]);
        //        println!("{:?}", s.map);
        assert_ne!(s.pick(), 7);
    }
}