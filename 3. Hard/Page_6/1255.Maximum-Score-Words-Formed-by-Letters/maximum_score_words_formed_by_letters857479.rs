// https://leetcode.com/problems/maximum-score-words-formed-by-letters/solutions/857479/rust-iterative-solution-0ms/
use std::collections::{HashMap, BTreeSet};
impl Solution {
    fn used_union(u1: &[u8; 26], u2: &[u8; 26]) -> [u8; 26] {
        let mut u = [0u8; 26];
        for ((x, y), z) in u1.iter().zip(u2.iter()).zip(u.iter_mut()) {
            *z = x + y;
        }
        u
    }
    
    fn next_combination(indicies: &mut [usize], n: usize) -> bool {
        let last = indicies.len() - 1;
        if indicies[last] == n - 1 {
            let mut j = last;
            let mut breaked = false;
            for k in 1..=last {
                j = last - k;
                if indicies[j] + 1 != indicies[j + 1] {
                    breaked = true;
                    break;
                }
            }
            if !breaked {
                return false;
            }
            indicies[j] += 1;
            let mut val = indicies[j];
            for i in &mut indicies[j+1..] {
                val += 1;
                *i = val;
            }
        } else {
           indicies[last] += 1;
        }
        
        true
    }
    
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        let mut map = [0u8; 26];
        let mut b = [0];
        for l in letters {
            l.encode_utf8(&mut b[..]);
            let idx = (b[0] - b'a') as usize;
            map[idx] += 1;
        }
        
        let mut overall_used = [0u8; 26];
        let mut overall_points = 0;
        let points: Vec<_> = words.into_iter().map(|w| {
            let mut used = [0u8; 26];
            let mut points = 0;
            for c in w.as_bytes().iter().copied() {
                let idx = (c - b'a') as usize;
                used[idx] += 1;
                if used[idx] <= map[idx] {
                    points += score[idx];
                }
            }
            
            overall_used = Self::used_union(&overall_used, &used);
            overall_points += points;
            (points, used)
        }).collect();
        
        if overall_used.iter().zip(map.iter()).all(|(&x, &y)| x <= y) {
            return overall_points;
        }
        
        let mut max_points = 0;
        for i in 1..points.len() {
            let mut indicies = (0..i).collect::<Vec<_>>();
            loop {
                let mut u = [0; 26];
                let mut sum = 0;
                for p in &indicies {
                    let p = &points[*p];
                    u = Self::used_union(&u, &p.1);
                    if u.iter().zip(map.iter()).any(|(&x, &y)| x > y) {
                        sum = 0;
                        break;
                    }
                    sum += p.0;
                }
                if sum > max_points {
                    max_points = sum;
                }
                
                if !Self::next_combination(&mut indicies, points.len()) {
                    break;
                }
            }
        }
        
        max_points
    }
}