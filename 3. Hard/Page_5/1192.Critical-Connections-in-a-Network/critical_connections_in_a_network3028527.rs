// https://leetcode.com/problems/critical-connections-in-a-network/solutions/3028527/rust-cpp-109-ms-24-6-mb-tarjan-s-algorithm/

// use std::borrow::Cow;
impl Solution {    
    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        
        let mut n = n as usize;
        let mut adj_list = vec![vec![]; n];
        
        for (start, end) in connections.iter().map(|f| (f[0] as usize, f[1] as usize)) {
            adj_list[start].push(end);
            adj_list[end].push(start);
        }
        let mut builder = SolutionBuilder::new(n, &adj_list);
        for node in 0..n { 
            if (builder.visited[node] == 0 ) { 
                builder.find_connections(node, -1);
            }
        } 
        builder.bridges
    }
}

#[derive(Debug, Clone)]
struct SolutionBuilder<'a> { 
    // We can use Cow here to clone the data 
    // adj_list: Cow<'a, Vec<Vec<usize>>>,
    adj_list: &'a Vec<Vec<usize>>,
    visited: Vec<usize>,
    time_in: Vec<usize>,
    time_out: Vec<usize>,
    bridges: Vec<Vec<i32>>,
    timer: usize
}

impl<'a> SolutionBuilder<'a> { 
    fn new(n: usize, adj_list: &'a Vec<Vec<usize>>) -> Self {
        let visited  = vec![0; n];
        let (time_in, time_out) = (vec![0; n], vec![0; n]);
        let bridges = vec![];
        let timer = 1;
        
        // let borrowed_list = Cow::Borrowed(adj_list);
        
        Self {
            // adj_list: borrowed_list,
            adj_list,

            visited, 
            time_in,
            time_out, 
            bridges, 
            timer
        }
    }
    
    fn find_connections(&mut self,  curr_node: usize,  parent_node: i32) {
        
        // Mark visited
        self.visited[curr_node] = 1;

        // Update the current time for this node 
        self.time_in[curr_node] = self.timer;
        self.time_out[curr_node] = self.timer;
        self.timer += 1;

        // Visit each neighbouring nodes
        for &next_node in self.adj_list[curr_node].iter() { 
            if parent_node == next_node as i32 {
                continue
            }
            if self.visited[next_node] == 0 { 
                self.find_connections(next_node, curr_node as i32);

                self.time_out[curr_node] = usize::min(
                    self.time_out[curr_node], 
                    self.time_out[next_node]
                );
                
                if self.time_out[next_node] > self.time_in[curr_node] { 
                    self.bridges.push(vec![curr_node as i32, next_node as i32]);
                }
            }
            self.time_out[curr_node] = usize::min(
                self.time_out[curr_node], 
                self.time_out[next_node]
            );
        }
    }
}