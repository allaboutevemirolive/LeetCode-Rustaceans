// https://leetcode.com/problems/group-anagrams/solutions/2751528/rust-solution/
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;
        let get_key = |s: &str| -> Vec<i32> {
            s.bytes().fold(vec![0; 26], |mut acc, b| {
                acc[(b - b'a') as usize] += 1;
                acc
            })
        };

        let mut hm: HashMap<Vec<i32>, Vec<String>> = HashMap::with_capacity(strs.len());
        strs.into_iter()
            .for_each(|s| hm.entry(get_key(s.as_str())).or_default().push(s));
        hm.into_values().collect()
    }
}