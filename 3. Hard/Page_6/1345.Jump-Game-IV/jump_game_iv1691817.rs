// https://leetcode.com/problems/jump-game-iv/solutions/1691817/rust-version/
impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        use std::collections::{HashMap,HashSet};
        let mut arrived:Vec<bool> = vec![false; arr.len()];
        let mut graph:HashMap<i32, Vec<usize>> = HashMap::new();
        for i in 0..arr.len() {
            let list = graph.entry(arr[i]).or_insert(vec![]);
            (*list).push(i);
        }
        let mut encounteredValue = HashSet::new();
        arrived[0] = true;
        let mut queue:Vec<(i32, i32)> = vec![(0, 0); 1];
        let mut res = 0;
        while queue.len() > 0 {
            let (index, dis) = queue[0];
            queue.remove(0);
            if index as usize == arr.len()-1 {
                res =  dis;
                break;
            }
            if index + 1 < arr.len() as i32 && arrived[index as usize + 1] == false {
                queue.push((index+1, dis+1));
                arrived[(index+1) as usize] = true;
            } 
            if index - 1 >= 0 && arrived[index as usize - 1] == false {
                queue.push((index-1, dis+1));
                arrived[(index-1) as usize] = true;
            }
            if encounteredValue.contains(&arr[index as usize]) == false {
                let mut vertex = &graph[&arr[index as usize]];
                for i in 0 .. vertex.len() {
                    if arrived[vertex[i]] == false && arr[vertex[i]] == arr[index as usize] {
                        queue.push((vertex[i] as i32, dis+1));
                        arrived[vertex[i]] = true;
                    }
                }
                graph.remove(&arr[index as usize]);
                encounteredValue.insert(arr[index as usize]);
            }
        }
        return res;
    }
}