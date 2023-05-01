// https://leetcode.com/problems/minimum-genetic-mutation/solutions/1766303/rust-adjacency-list-bfs/
use std::collections::VecDeque;

const MUTATION_NOT_POSSIBLE: i32 = -1;
const START_GENE: usize = 0;

pub fn min_mutation(start: String, end: String, mut bank: Vec<String>) -> i32 {
    // make sure that "start" is in the 0th position in "bank"
    let mut bank_contains_start = false;
    for idx in 0..bank.len() {
        if start == bank[idx] {
            bank.swap(START_GENE, idx);
            bank_contains_start = true;
            break;
        }
    }

    if !bank_contains_start {
        bank.push(start);
        let last = bank.len() - 1;
        bank.swap(START_GENE, last);
    }

    // Build the adjacency lists
    let mut graph = vec![vec![]; bank.len()];
    for i in 0..bank.len() {
        for j in i + 1..bank.len() {
            if is_adjacent(&bank[i], &bank[j]) {
                graph[i].push(j);
                graph[j].push(i);
            }
        }
    }

    // BFS
    let mut queue = VecDeque::new();
    let mut visited = vec![false; graph.len()];

    queue.push_back((START_GENE, 0));
    while let Some((idx, cost)) = queue.pop_front() {
        if visited[idx] {
            continue;
        }

        if end == bank[idx] {
            return cost;
        }

        visited[idx] = true;
        for &i in graph[idx].iter() {
            if !visited[i] {
                queue.push_back((i, cost + 1));
            }
        }
    }

    MUTATION_NOT_POSSIBLE
}

fn is_adjacent(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let a = a.as_bytes();
    let b = b.as_bytes();

    let mut diffs = 0;
    for idx in 0..a.len() {
        if a[idx] != b[idx] {
            diffs += 1;

            if diffs > 1 {
                break;
            }
        }
    }

    diffs == 1
}