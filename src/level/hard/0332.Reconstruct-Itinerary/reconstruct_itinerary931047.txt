// https://leetcode.com/problems/reconstruct-itinerary/solutions/931047/rust-easy-greedy-solution-4ms/
use std::collections::HashMap;

fn lookup(
        target_len:usize, 
        current_len: usize, 
        prev: &str, 
        mapping: &mut HashMap<String, Vec<String>>
)->Option<Vec<String>>{
    if target_len == current_len{
        return Some(Vec::with_capacity(target_len));
    }
    
    let len = mapping.get_mut(prev)?.len();
    // Because we sorted greedily
    // most of the time this iteration stop on first iteration
    // It is needed, if we cannot construct path from last position.
    for nxt in (0..len).rev(){
        let removed = mapping.get_mut(prev)?.remove(nxt);
        let mut res_nxt = lookup(target_len, current_len + 1, &removed, mapping);
        if let Some(mut res) = res_nxt{
            res.push(removed.clone());
            mapping.get_mut(prev)?.insert(nxt, removed);
            return Some(res);
        }
        mapping.get_mut(prev)?.insert(nxt, removed);
    }
    None
}

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let target_len = tickets.len();
        
        let mut mapping: HashMap<String, Vec<String>> = HashMap::new();
        for mut p in tickets{
            let t = p.pop().unwrap();
            let s = p.pop().unwrap();
            mapping.entry(s).or_default().push(t);
        }
        for v in mapping.values_mut(){
            v.sort_unstable_by(|a,b|b.cmp(a));
        }
        
        let mut res = lookup(target_len, 0, "JFK", &mut mapping).expect("Should be solution");
        res.push("JFK".to_string());
        res.reverse();
        res
    }
}