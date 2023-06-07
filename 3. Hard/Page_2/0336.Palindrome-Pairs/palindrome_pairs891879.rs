// https://leetcode.com/problems/palindrome-pairs/solutions/891879/64-ms-rust-solution-beats-100/
fn mk_idx(v: u8)->usize{
    (v-b'a') as usize
}

impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
	    // Need this ugly initialization because [x; N] require x to be copyable :(
		// Mapping from last letter to indices in `words`
        let mut last_let_to_indices = [
            Vec::new(),Vec::new(),Vec::new(),
            Vec::new(),Vec::new(),Vec::new(),
            Vec::new(),Vec::new(),Vec::new(),
            Vec::new(),Vec::new(),Vec::new(),
            Vec::new(),Vec::new(),Vec::new(),
            Vec::new(),Vec::new(),Vec::new(),
            Vec::new(),Vec::new(),Vec::new(),
            Vec::new(),Vec::new(),Vec::new(),
            Vec::new(),Vec::new()
        ];
		// Flag for possible empty string
        let mut empty_idx = None;
        for (i, w) in words.iter().map(String::as_bytes).enumerate(){
            match w.last(){
                Some(&c)=>last_let_to_indices[mk_idx(c)].push(i),
                None=>empty_idx = Some(i),
            }
        }
        
        let mut results = Vec::new();
        // Improved O(n^2)
        for (i, w1) in words.iter().map(String::as_bytes).enumerate(){
		    // We will handle it later
            if w1.is_empty(){
                continue;
            }
            let first = mk_idx(*w1.first().unwrap());
            for &j in last_let_to_indices[first].iter(){
                if i==j{
                    continue;
                }
                let w2 = words[j].as_bytes();
                let sum_len_half =(w1.len() + w2.len())/2;
				// Rust iterators power allows to avoid extra allocation here
                let iter = w1.iter().copied().chain(w2.iter().copied());
                let reversed = iter.clone().rev();
                for (idx, (c1, c2)) in iter.zip(reversed).enumerate(){
                    if c1 != c2{
                        break;
                    }
                    if idx >= sum_len_half{
                        results.push(vec![i as i32, j as i32]);
                        break;
                    }
                }
            }
        }
        
        // Case when one of words is empty
        // O(n)
        if let Some(empty_idx) = empty_idx {
            for (i, w) in words.iter().map(String::as_bytes).enumerate(){
                if i==empty_idx{
                    continue;
                }
                let len_half = w.len() / 2;
                let iter = w.iter().copied();
                let reversed = iter.clone().rev();
                for (idx, (c1, c2)) in iter.zip(reversed).enumerate(){
                    if c1 != c2{
                        break;
                    }
                    if idx >= len_half{
                        results.push(vec![i as i32, empty_idx as i32]);
                        results.push(vec![empty_idx as i32, i as i32]);
                        break;
                    }
                }
            }
        }
        
        results
    }
}