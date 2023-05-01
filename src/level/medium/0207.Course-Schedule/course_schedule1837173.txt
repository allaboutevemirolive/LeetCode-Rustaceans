// https://leetcode.com/problems/course-schedule/solutions/1837173/rust-two-solutions-dfs-cycle-detection-kahn-s-algorithm/
#[derive(Copy, Clone, Eq, PartialEq)]
enum Marker {
    Unvisited,
    Undecided,
    Processed,
}

pub fn can_finish(num_courses: usize, prerequisites: Vec<Vec<i32>>) -> bool {
    // build the graph from the given edges
    let mut graph = vec![vec![]; num_courses];
    for edge in prerequisites.into_iter() {
        graph[edge[0] as usize].push(edge[1] as usize);
    }

    // because the graph may not be connected we have to go over all nodes
    let mut visited = vec![Marker::Unvisited; num_courses];
    for node in 0..num_courses {
        // If we have already processed this node, then skip it in order to
        // save time - we already know that it does not contain a cycle
        if visited[node] == Marker::Unvisited {
            if has_cycle(&graph, &mut visited, node) {
                return false;
            }
        }
    }

    true
}

fn has_cycle(graph: &[Vec<usize>], visited: &mut [Marker], node: usize) -> bool {
    match visited[node] {
        // We are in a cycle
        Marker::Undecided => return true,
        // We have already processed that subtree and it does not contain a cycle
        Marker::Processed => return false,
        // Check if the current subtree is part of a cycle. We set this state 
        // only on unvisited nodes. If we visit them again - then we are in a 
        // cycle, otherwise we mark them as "Processed"
        Marker::Unvisited => visited[node] = Marker::Undecided,
    }

    for &next in graph[node].iter() {
        if has_cycle(graph, visited, next) {
            return true;
        }
    }

    visited[node] = Marker::Processed;
    false
}