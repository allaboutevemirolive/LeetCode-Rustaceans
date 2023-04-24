// https://leetcode.com/problems/frog-position-after-t-seconds/solutions/931125/rust-simple-using-pathfinding-4-ms/
use std::collections::{HashMap, HashSet};
use std::convert::{TryFrom, TryInto};

fn find_path(current: i32, target: i32, prev: i32, graph: &HashMap<i32, Vec<i32>>)->Option<Vec<i32>>{
    if current == target{
        return Some(vec![current]);
    }
    for &next in graph[&current].iter(){
        if next == prev{
            continue;
        }
        if let Some(mut r) = find_path(next, target, current, graph){
            r.push(current);
            return Some(r);
        }
    }
    None
}

impl Solution {
    pub fn frog_position(n: i32, edges: Vec<Vec<i32>>, t: i32, target: i32) -> f64 {
        let graph: HashMap<i32, Vec<i32>> = {
            let mut graph: HashMap<i32, HashSet<i32>> = HashMap::with_capacity(edges.len()*2);
            graph.insert(1, HashSet::new());
            for edge in edges{
                graph.entry(edge[1]).or_default().insert(edge[0]);
                graph.entry(edge[0]).or_default().insert(edge[1]);
            }
            graph.into_iter().map(|(k, v)|{
                let mut v: Vec<i32> = v.into_iter().collect();
                v.sort_unstable();
                (k, v)
            }).collect()
        };
        
        let mut path = find_path(1, target, 1, &graph).expect("Trees always have path");
        path.reverse();
        let path = path;
        
        assert_eq!(path[0], 1);
        if path.len() > usize::try_from(t).unwrap() + 1 {
            return 0.0;
        }
        if path.len() == 1{
            return if t == 0 || graph[&1].len()==0 { 1.0 } else { 0.0 };
        }
        let mut prob = 1.0f64 / graph[&1].len() as f64;
        for &current in path[1..].iter(){
            if current == target{
                if path.len() == usize::try_from(t).unwrap() + 1 || graph[&current].len() == 1{
                    return prob;
                }
                else{
                    return 0.0;
                }
            }
            prob = prob / (graph[&current].len() - 1) as f64;
        }
        unreachable!()
    }
}