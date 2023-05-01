// https://leetcode.com/problems/course-schedule-ii/solutions/3406423/rust-version-of-topological-sort/
use std::collections::HashMap;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {

        if prerequisites.len() == 0 {
            return (0..num_courses).collect();
        }
        let mut res: Vec<i32> = vec![];
        let default_list: Vec<i32> = Vec::new();

        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut parents = vec![0; num_courses as usize];
        let mut queue: Vec<i32> = vec![];

        for edge in prerequisites {
            let a = edge[0];
            let b = edge[1];
            graph.entry(b).or_insert(Vec::new()).push(a);
            parents[a as usize] += 1;
        }

        for i in 0..parents.len() {
            if parents[i] == 0 {
                queue.push(i as i32);
            }
        }

        while !queue.is_empty() {
            let e = queue.remove(0);
            res.push(e);
            let list = graph.get(&e).unwrap_or(&default_list);

            for v in list.into_iter() {
                parents[*v as usize] -= 1; 
                if parents[*v as usize] == 0 {
                    queue.push(*v);
                }
            }

        }

        if res.len() as i32 == num_courses {
            return res;
        }
        return default_list;
    }
}
