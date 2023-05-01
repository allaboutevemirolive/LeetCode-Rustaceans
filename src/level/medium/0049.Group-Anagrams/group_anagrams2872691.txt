// https://leetcode.com/problems/group-anagrams/solutions/2872691/solution-in-rust/

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<&String>> = HashMap::new();

        for el in strs.iter() {
            let mut cp = el.chars().collect::<Vec<char>>();
			// unstable sort is known to be faster.
            cp.sort_unstable();
            let entry = map.entry(cp.into_iter().collect::<String>())
                .or_insert(vec![]);
            entry.push(el);
        }

        map.values()
            .map(|v| v.into_iter()
                .map(|v| (*v).to_owned())
                .collect::<Vec<String>>()
            )
            .collect()
    }
}