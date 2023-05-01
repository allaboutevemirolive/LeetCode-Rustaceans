// https://leetcode.com/problems/evaluate-division/solutions/1993594/rust-bfs/
use std::collections::HashMap;
use std::collections::VecDeque;

impl Solution {
    pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
        let mut graph: HashMap<String, Vec<(String, f64)>> = HashMap::new();

        for i in 0..equations.len() {
            if let Some(neighbors) = graph.get_mut(&equations[i][0]) {
                neighbors.push((equations[i][1].clone(), values[i]));
            }
            else {
                graph.insert(equations[i][0].clone(), vec![(equations[i][1].clone(), values[i])]);
            }

            if let Some(neighbors) = graph.get_mut(&equations[i][1]) {
                neighbors.push((equations[i][0].clone(), 1.0/values[i]));
            }
            else {
                graph.insert(equations[i][1].clone(), vec![(equations[i][0].clone(), 1.0/values[i])]);
            }
        }
        
        queries.iter()
            .map(|q| Solution::bfs(&graph, &q[0], &q[1]))
            .collect()
    }

    fn bfs(graph: &HashMap<String, Vec<(String, f64)>>, start: &String, goal: &String) -> f64 {

        if graph.get(start) == None {
            return -1.0;
        }

        if start == goal {
            return 1.0;
        }

        let mut parents = HashMap::new();
        let mut queue = VecDeque::new();
        let mut visited = HashMap::new();

        queue.push_back(start);
        visited.insert(start, true);
        parents.insert(start, start);

        while let Some(next) = queue.pop_front() {
            if next == goal {
                return Solution::compute_equation(graph, start, goal, &parents);
            }

            if let Some(neighbors) = graph.get(next) {
                for neighbor in neighbors {
                    if let Some(_) = visited.get(&neighbor.0) {
                        continue;
                    }

                    queue.push_back(&neighbor.0);
                    visited.insert(&neighbor.0, true);
                    parents.insert(&neighbor.0, next);
                }
            }
        }

        -1.0
    }

    fn compute_equation(graph: &HashMap<String, Vec<(String, f64)>>, start: &String, goal: &String, parents: &HashMap<&String, &String>) -> f64 {
        let mut values = Vec::new();
        let mut parent = goal;

        while parent != start {
            let new_parent = *parents.get(parent).unwrap();

            for neighbor in graph.get(new_parent).unwrap() {
                if neighbor.0 == *parent {
                    values.push(neighbor.1);
                    break;
                }
            }
            parent = new_parent;
        }

        values.iter().fold(1.0, |acc, val| acc*val)
    }
}