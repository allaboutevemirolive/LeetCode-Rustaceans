// https://leetcode.com/problems/group-anagrams/solutions/2751760/rust-intuitive-hashmap/
use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::new();
        
        for i in 0..strs.len() {
            let sorted = Self::sort_string(strs[i].clone());
            
            map.entry(sorted)
                .and_modify(|list: &mut Vec<String>| list.push(strs[i].to_owned()))
                .or_insert(vec![strs[i].to_owned()]);
        }
        
        map.into_values().collect()
    }
    
    fn sort_string(st: String) -> String {
        let mut chars = st.chars().collect::<Vec<char>>();
        
        chars.sort_by(|a, b| b.cmp(a));
        
        chars.into_iter().collect::<String>()
    }
}