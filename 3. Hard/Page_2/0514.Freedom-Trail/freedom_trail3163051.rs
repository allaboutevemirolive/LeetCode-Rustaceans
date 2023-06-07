// https://leetcode.com/problems/freedom-trail/solutions/3163051/rust-java-6-ms-dp-from-lee215/
use std::cmp::min;
use std::collections::HashMap;

impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
		let position = ring.as_bytes().iter().enumerate().fold(vec![vec![]; 26], |mut acc, (i, c)| {  
		    acc[(c - b'a') as usize].push(i);  
		    acc  
		});
        fn distance(i: usize, j: usize, ring: &str) -> usize {
            let abs = (i as i32 - j as i32).abs() as usize;
            let len = ring.len();
            min(abs, len - abs)
        }
        // divide key into chars
        let state = key.chars().into_iter().fold(HashMap::from([(0, 0)]), |mut curr, c| {
            // every possible target position
            position[(c as u8 - b'a') as usize].iter().fold(HashMap::new(), |mut next, &target| {
                let count = next.entry(target).or_insert(usize::MAX);
                // every possible start position
                for (&start, dis) in &curr {
                    *count = min(*count, dis + distance(target, start, &ring));
                }
                next
            })
        });
        (state.values().min().unwrap() + key.len()) as i32
    }
}