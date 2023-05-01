// https://leetcode.com/problems/group-anagrams/solutions/2753111/rust-solution/
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;

        let mut result: Vec<Vec<String>> = vec![];
        let mut str_map: HashMap<Vec<char>, Vec<String>> = HashMap::new();
        strs.iter().for_each(|str| {
            let mut key = str.chars().collect::<Vec<char>>();
            key.sort_unstable();
            str_map
                .entry(key)
                .and_modify(|vec| vec.push(str.clone()))
                .or_insert(vec![str.clone()]);
        });
        str_map.iter().for_each(|(_, vec)| result.push(vec.clone()));

        result
    }
}