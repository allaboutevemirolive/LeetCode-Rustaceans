// https://leetcode.com/problems/russian-doll-envelopes/solutions/1345208/rust-1616-ms-topological-sort-solution-o-n-2/
use std::collections::VecDeque;

impl Solution {
    pub fn topological_sort(N: usize, graph: &Vec<Vec<usize>>) -> Vec<usize> {
        let mut indegree = vec![0; N];
        let mut queue = VecDeque::new();
        for i in 0..N {
            for j in graph[i].iter() {
                indegree[*j] += 1;
            }
        }
        for i in 0..N {
            if indegree[i] == 0 {
                queue.push_back(i);
            }
        }

        let mut res = Vec::new();
        while 0 < queue.len() {
            let u = queue.pop_front().unwrap();
            res.push(u);
            for v in graph[u].iter() {
                indegree[*v] -= 1;
                if indegree[*v] == 0 {
                    queue.push_back(*v)
                }
            }
        }
        return res    
    }
    
    pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
        let N = envelopes.len();
        let mut graph = vec![vec![]; N];
        for i in 0..N {
            let u = &envelopes[i];
            for j in 0..N {
                if i == j {
                    continue
                }
                let v = &envelopes[j];
                if (u[0] > v[0]) && (u[1] > v[1]) {
                    graph[i as usize].push(j);
                }
            }
        }
        
        let order = Self::topological_sort(N, &graph);
        let mut dist = vec![-1; N];
        for u in order.iter() {
            if dist[*u] != -1 {
                continue
            }
            dist[*u] = 1;
            let mut queue = VecDeque::new();
            queue.push_back(*u);
            while 0 < queue.len() {
                let v = queue.pop_front().unwrap();
                let prev = dist[v];
                for w in graph[v].iter() {
                    if dist[*w] < prev + 1 {
                        dist[*w] = prev + 1;
                        queue.push_back(*w)
                    }
                }
            }
        }
        
        return *dist.iter().max().unwrap()
    }
}