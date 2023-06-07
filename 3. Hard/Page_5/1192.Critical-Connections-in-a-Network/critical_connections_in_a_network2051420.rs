// https://leetcode.com/problems/critical-connections-in-a-network/solutions/2051420/rust-tarjan-s-algorithm/
pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    assert!(n > 0);

    let mut graph = vec![vec![]; n as usize];
    for connection in connections.iter() {
        graph[connection[0] as usize].push(connection[1] as usize);
        graph[connection[1] as usize].push(connection[0] as usize);
    }

    let mut answer = vec![];
    let mut discovery = vec![0; n as usize];
    let mut earliest = vec![0; n as usize];

    dfs(
        &graph,
        &mut answer,
        &mut discovery,
        &mut earliest,
        1,
        n as usize,
        0,
    );

    answer
}

fn dfs(
    graph: &[Vec<usize>],
    answer: &mut Vec<Vec<i32>>,
    discovery: &mut [usize],
    earliest: &mut [usize],
    counter: usize,
    parent: usize,
    node: usize,
) {
    // Add a guard to avoid visiting a node for a second time
    // if discovery[node] != 0 {
    //     return;
    // }

    discovery[node] = counter;
    earliest[node] = counter;

    for &child in graph[node].iter() {
        if child == parent {
            continue;
        }

        // a discovery tim eof 0, means that the node has not been visited yet
        if discovery[child] == 0 {
            dfs(graph, answer, discovery, earliest, counter + 1, node, child);

            earliest[node] = earliest[node].min(earliest[child]);
            if discovery[node] < earliest[child] {
                answer.push(vec![node as i32, child as i32]);
            }
        } else {
            earliest[node] = earliest[node].min(discovery[child]);
        }
    }
}