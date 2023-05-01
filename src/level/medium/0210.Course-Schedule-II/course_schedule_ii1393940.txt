// https://leetcode.com/problems/course-schedule-ii/solutions/1393940/kahn-algo-rust/
use std::{collections::{VecDeque}, vec};

struct Graph {
    pub node_count : usize,
    pub neighbours : Vec<Vec<usize>>
}

impl Graph {

    pub fn new(count : usize) -> Self {

        let neighbour : Vec<Vec<usize>> = vec![vec![]; count];
        return Graph{
            node_count : count,  
            neighbours : neighbour
        }
    }

    pub fn add_edge(&mut self, start_node : usize, end_edge : usize) {
        self.neighbours[start_node].push(end_edge);
    }
}

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph = Graph::new(num_courses as usize);
        let mut result : Vec<i32> = Vec::new();
        let mut q : VecDeque<usize> = VecDeque::new();
        let mut indegree :Vec<usize> = vec![0; num_courses as usize];
        for item in prerequisites.iter() {
            graph.add_edge(item[1] as usize, item[0] as usize);
        }

        for i in graph.neighbours.iter() {        
            for j in i.iter(){
                indegree[*j as usize] = indegree[*j as usize] + 1;
            }        
        }
        
        for i in 0..num_courses {
            if indegree[i as usize] == 0 {
                q.push_back(i as usize);
            }
        }

        while !q.is_empty() {
           let node = q.pop_front().unwrap();
           result.push(node as i32);
           for nbr in graph.neighbours[node].iter() {
                indegree[*nbr] = indegree[*nbr] - 1;
                println!("{:?}", indegree[*nbr]);
                if indegree[*nbr] == 0 {
                    q.push_back(*nbr);
                }
            }
        }

        return if result.len() == num_courses as usize { result } else {Vec::new()};
    }
}
