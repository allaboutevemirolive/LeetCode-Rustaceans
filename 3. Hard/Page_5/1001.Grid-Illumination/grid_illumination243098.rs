// https://leetcode.com/problems/grid-illumination/solutions/243098/rust-hashmaps/
use std::collections::{HashMap, HashSet};

struct LampGrid {
    horizontal : HashMap<i64, i32>,
    vertical : HashMap<i64, i32>,
    rdiagonals : HashMap<i64, i32>,
    ldiagonals : HashMap<i64, i32>,
    lamps : HashSet<(usize, usize)>,
    n : usize,
}

impl LampGrid {
    pub fn new(n : usize, lamps : Vec<Vec<i32>>) -> Self {
		// we keep track of which lines (horizontal, vertical, ldiagonal, rdiagonal) contain a lamp
        let mut horizontal = HashMap::new();
        let mut vertical = HashMap::new();
        let mut rdiagonals = HashMap::new();
        let mut ldiagonals = HashMap::new();
        
        for lamp in lamps.iter() {
            let (x,y) = (lamp[0] as i64, lamp[1] as i64);
            
            *horizontal.entry(y).or_insert(0) += 1;
            *vertical.entry(x).or_insert(0) += 1;
            *ldiagonals.entry(y-x).or_insert(0) += 1;
            *rdiagonals.entry(x+y).or_insert(0) += 1;
        }
        
        LampGrid {
            horizontal,
            vertical,
            ldiagonals,
            rdiagonals,
            lamps : lamps.into_iter().map(|v| (v[0] as usize, v[1] as usize)).collect(),
            n,
        }
    }
    
    fn is_illuminated(&self, x : i64, y : i64) -> bool {
		// does this position lie on a line containing a lamp?
        (self.horizontal.contains_key(&y) && self.horizontal[&y] > 0) || 
        (self.vertical.contains_key(&x) && self.vertical[&x] > 0) ||
        (self.rdiagonals.contains_key(&(x+y)) && self.rdiagonals[&(x+y)] > 0) ||
        (self.ldiagonals.contains_key(&(y-x)) && self.ldiagonals[&(y-x)] > 0)
    }
    
    fn dim(&mut self, x : i64, y : i64) {
        if self.lamps.remove(&(x as usize, y as usize)) {
			// decrease the lamp count on all of the lines this lamp is on
            *(self.horizontal.get_mut(&y).unwrap()) -= 1;
            *(self.vertical.get_mut(&x).unwrap()) -= 1;
            *(self.rdiagonals.get_mut(&(x+y)).unwrap()) -= 1;
            *(self.ldiagonals.get_mut(&(y-x)).unwrap()) -= 1;
        }
    }
	
    pub fn query(&mut self, x : i64, y : i64) -> i32 {
        let answer = if self.is_illuminated(x,y) { 1 } else { 0 };
        
        // dim all lights that are at or adjacent 8-directionally to (x,y)
        for xdelta in -1..=1 {
            for ydelta in -1..=1 {
                self.dim(x + xdelta, y + ydelta);
            }
        }
        
        answer
    }
}

impl Solution {
    pub fn grid_illumination(n: i32, lamps: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut lg = LampGrid::new(n as usize, lamps);
        
        queries.into_iter()
               .map(|v| lg.query(v[0] as i64, v[1] as i64))
               .collect()
    }
}