// https://leetcode.com/problems/course-schedule/solutions/1381376/simple-dfs-rust/

struct Graph {
    pub node_count : i32,
    pub neighbours : Vec<Vec<i32>>,
}

impl Graph {

    pub fn new(count : i32) -> Self {

        let neighbour : Vec<Vec<i32>> = vec![vec![]; count as usize];
        return Graph{
            node_count : count,  
            neighbours : neighbour
        }
    }

    pub fn add_edge(&mut self, start_node : usize, end_edge : usize) {
        self.neighbours[start_node].push(end_edge as i32);
    }

    pub fn contains_cycle(&self) -> bool {

        fn dfs(node : i32, graph : &Graph, visited : &mut Vec<bool>, stack : &mut Vec<bool>) -> bool {

            visited[node as usize] = true;
            stack[node as usize] = true;

            for nbr in graph.neighbours[node as usize].iter() {
                let deref_nbr = *nbr;

                if stack[deref_nbr as usize] {
                    return true;
                } else if visited[deref_nbr as usize] {
                    let nbr_found_cycle = dfs(deref_nbr, graph, visited, stack);
                    if nbr_found_cycle {
                        return true;
                    }
                }

            }
            stack[node as usize] = false;
            return false;
        }

        let mut visited_nodes = vec![false; self.node_count as usize];

        let mut stack = vec![false; self.node_count as usize];

        for i in 0 .. self.node_count {
            if !visited_nodes[i as usize] {
                let result = dfs(i, &self, &mut visited_nodes, &mut stack);
                if result {
                    return false;
                }
            }
        }
        return true;

    }
}

impl Solution {
  pub fn can_finish(num_courses: i32, r: Vec<Vec<i32>>) -> bool {
      
     let mut graph = Graph::new(num_courses);
      for i in r.iter() {
          graph.add_edge(i[1] as usize, i[0] as usize);
      }
    return graph.contains_cycle();
}
}