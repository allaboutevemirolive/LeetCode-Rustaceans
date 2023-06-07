// https://leetcode.com/problems/similar-string-groups/solutions/3463851/rust-dfs/
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        let n = strs.len();
        let mut adj_list: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut visited: HashSet<i32> = HashSet::new();
        let mut count: i32 = 0;
        
        fn is_similar(a: &str, b: &str) -> bool {
            let mut diff = 0;
            for (i,c) in a.chars().enumerate(){
                if c != b.chars().nth(i).unwrap() {
                    diff += 1;
                }
            }
            diff == 0 || diff == 2
        }
        
        for i in 0..n {
            for j in i+1..n {
                if is_similar(&strs[i],&strs[j]) {
                    adj_list.entry(i as i32).or_insert(Vec::new()).push(j as i32);
                    adj_list.entry(j as i32).or_insert(Vec::new()).push(i as i32);
                }
            }
        }
        
        fn dfs(node: i32, visited: &mut HashSet<i32>, adj_list: &HashMap<i32, Vec<i32>>) {
            visited.insert(node);
            for &nei in adj_list.get(&node).unwrap_or(&Vec::new()) {
                if(!visited.contains(&nei)) {
                    dfs(nei,visited,adj_list);
                }
            }
        }
        
        for i in 0..n{
            if !visited.contains(&(i as i32)) {
                dfs(i as i32,&mut visited, &adj_list);
                count += 1
            }
        }
        
        return count;
    }
}