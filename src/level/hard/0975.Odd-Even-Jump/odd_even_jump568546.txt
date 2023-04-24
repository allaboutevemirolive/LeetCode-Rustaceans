// https://leetcode.com/problems/odd-even-jump/solutions/568546/rust-btreemap/
use std::collections::BTreeMap; //  BTreeMap::range is the key to this solution's performance

impl Solution {
    pub fn odd_even_jumps(a: Vec<i32>) -> i32 {
        
        if a.is_empty() {
            return 0;
        }
		
		// the keys are the values from the array
		// the values are (whether an odd jump would be successful, whether an even jump would be successful)
		let mut seen: BTreeMap<i32, (bool, bool)> = BTreeMap::new();
		
		// we're always successful from the last position
        seen.insert(*a.last().unwrap(), (true, true));
		let mut count = 1;
        
        for &value in a.iter().rev().skip(1) {
            let odd_jump_ok = if let Some((_, &(_, successor_even_ok))) = seen.range(value..).next() {
               successor_even_ok
            } else {
                false
            };
            let even_jump_ok = if let Some((_, &(predecessor_odd_ok, _))) = seen.range(..=value).last() {
                predecessor_odd_ok
            } else {
                false
            };
            if odd_jump_ok {
                count += 1;
            }
            seen.insert(value, (odd_jump_ok, even_jump_ok));
        }
        
        count
    }
}