// https://leetcode.com/problems/verbal-arithmetic-puzzle/solutions/1499630/rust-solution-with-backtracking-and-pruning-304-ms-2-mb/
use std::collections::{HashSet, HashMap};

fn dot(w: &[usize], mapping: &[usize]) -> usize {
    let mut ans = 0;
    for i in (0..w.len()) {
        ans += w[i]*mapping[i];
    }
    ans
}

fn mapping_possible(words: &[usize], res: &[usize], leading: &[bool], mapping: &mut [usize], used: &mut [bool], num_chars: usize, idx: usize) -> bool {
    if num_chars == idx {
        dot(words, mapping) == dot(res, mapping)
    } else {
        for i in (idx..num_chars) { mapping[i] = 0; }
        let small_words = dot(words, mapping);
        let small_res = dot(res, mapping);
        for i in (idx..num_chars) { mapping[i] = 9; }
        let big_words = dot(words, mapping);
        let big_res = dot(res, mapping);
        if big_words < small_res || big_res < small_words {
            return false;
        }
        
        let mut process_dig = |dig| {
            if leading[idx] && dig == 0 { return false; }
            if used[dig] { return false; }
            
            mapping[idx] = dig;
            used[dig] = true;
            if mapping_possible(words, res, leading, mapping, used, num_chars, idx+1) { return true; }
            
            used[dig] = false;
            false
        };
        
        if idx == num_chars-1 {
            let numer = (small_words as isize)-(small_res as isize);
            let denom = (res[idx] as isize)-(words[idx] as isize);
            if denom != 0 {
                if numer % denom == 0 {
                    let last_dig = numer/denom;
                    return last_dig >= 0 && last_dig <= 9 && process_dig(last_dig as usize);
                }
                else { return false; }
            }
        }
        
        (0..=9).any(process_dig)
    }
}

fn add_word(interp: &mut HashMap<char, usize>, leading_chs: &mut HashSet<usize>, word: &str) {
    for (i, ch) in word.chars().enumerate() {
        let sz = interp.len();
        let res = interp.entry(ch).or_insert(sz);
        if i == 0 && word.len() > 1 { leading_chs.insert(*res); }
    }
}

impl Solution {
    pub fn is_solvable(words: Vec<String>, result: String) -> bool {
        let mut interp_chars = HashMap::new();
        let mut leading_chars = HashSet::new();
        for word in words.iter() {
            add_word(&mut interp_chars, &mut leading_chars, word);
        }
        add_word(&mut interp_chars, &mut leading_chars, &result);
        
        let new_words: Vec<Vec<usize>> = words.into_iter()
        .map(|s| s.chars().map(|ch| interp_chars[&ch]).collect())
        .collect();
        let res: Vec<usize> = result.chars().map(|ch| interp_chars[&ch]).collect();
        let mut mapping: Vec<usize> = vec![0; interp_chars.len()];
        let mut used: Vec<bool> = vec![false; 10];
        let leading: Vec<bool> = (0..interp_chars.len()).map(|dig| leading_chars.contains(&dig)).collect();
        
        let mut words_value = vec![0; interp_chars.len()];
        for word in new_words.iter() {
            for (i, &ch) in word.iter().enumerate() {
                words_value[ch] += 10_usize.pow((word.len()-i-1) as u32);
            }
        }
        let mut res_value = vec![0; interp_chars.len()];
        for (i, &ch) in res.iter().enumerate() {
            res_value[ch] += 10_usize.pow((res.len()-i-1) as u32);
        }
        
        mapping_possible(&words_value, &res_value, &leading, &mut mapping, &mut used, interp_chars.len(), 0)
    }
}