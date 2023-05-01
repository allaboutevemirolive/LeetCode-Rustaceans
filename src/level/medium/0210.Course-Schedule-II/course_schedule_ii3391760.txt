// https://leetcode.com/problems/course-schedule-ii/solutions/3391760/rust-2-solutions-dfs-and-topological-sort-clean-readable-code-with-tests/
impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        get_order(num_courses as usize, &prerequisites, Algorithm::TopologicalSort)
    }
}

use std::collections::VecDeque;

pub enum Algorithm {
    Dfs,
    TopologicalSort,
}

#[derive(Clone, PartialEq, Eq)]
enum Status {
    Red,    // Unvisited
    Yellow, // In progress
    Green,  // Completed
}

pub fn get_order(vertices: usize, edges: &[Vec<i32>], alg: Algorithm) -> Vec<i32> {
    let edges = edges
        .iter()
        .map(|courses| (courses[0] as usize, courses[1] as usize))
        .collect::<Vec<_>>();

    let order = match alg {
        Algorithm::Dfs => get_order_dfs(vertices, &edges),
        Algorithm::TopologicalSort => get_order_topological_sort(vertices, &edges),
    };

    order.into_iter().map(|x| x as i32).collect()
}

pub fn get_order_dfs(vertices: usize, edges: &[(usize, usize)]) -> Vec<usize> {
    let graph = build_graph(vertices, edges);
    let mut statuses = vec![Status::Red; vertices];

    let mut order = vec![];

    for v in 0..vertices {
        if statuses[v] == Status::Red && has_cycle(v, &mut statuses, &graph, &mut order) {
            return vec![];
        }
    }

    order.reverse();
    order
}

pub fn get_order_topological_sort(vertices: usize, edges: &[(usize, usize)]) -> Vec<usize> {
    let graph = build_graph(vertices, edges);
    let mut indegrees = build_indegrees(vertices, edges);

    let mut q = indegrees
        .iter()
        .enumerate()
        .filter_map(|(course, indegree)| match *indegree {
            0 => Some(course),
            _ => None,
        })
        .collect::<VecDeque<_>>();

    let mut order = vec![];

    while !q.is_empty() {
        if let Some(first) = q.pop_front() {
            order.push(first);

            for &second in &graph[first] {
                indegrees[second] -= 1;

                if indegrees[second] == 0 {
                    q.push_back(second);
                }
            }
        }
    }

    match order.len() == vertices {
        true => order,
        false => vec![],
    }
}

fn has_cycle(
    course: usize,
    statuses: &mut [Status],
    graph: &[Vec<usize>],
    order: &mut Vec<usize>,
) -> bool {
    match statuses[course] {
        Status::Green => false,
        Status::Yellow => true,
        Status::Red => {
            statuses[course] = Status::Yellow;

            if graph[course]
                .iter()
                .any(|&next_course| has_cycle(next_course, statuses, graph, order))
            {
                return true;
            }

            statuses[course] = Status::Green;
            order.push(course);

            false
        }
    }
}

fn build_graph(vertices: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; vertices];

    for &(dst, src) in edges {
        graph[src].push(dst);
    }

    graph
}

fn build_indegrees(vertices: usize, edges: &[(usize, usize)]) -> Vec<usize> {
    let mut indegrees = vec![0; vertices];

    for &(dst, _) in edges {
        indegrees[dst] += 1;
    }

    indegrees
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_order_dfs_1() {
        assert_eq!(get_order_dfs(2, &[(1, 0)]), vec![0, 1]);
    }

    #[test]
    fn test_get_order_dfs_2() {
        assert_eq!(
            get_order_dfs(4, &[(1, 0), (2, 0), (3, 1), (3, 2)]),
            vec![0, 2, 1, 3]
        );
    }

    #[test]
    fn test_get_order_dfs_3() {
        assert_eq!(get_order_dfs(1, &[]), vec![0]);
    }

    #[test]
    fn test_get_order_dfs_4() {
        assert_eq!(get_order_dfs(2, &[(1, 0), (0, 1)]), vec![]);
    }

    #[test]
    fn test_get_order_topological_sort_1() {
        assert_eq!(get_order_topological_sort(2, &[(1, 0)]), vec![0, 1]);
    }

    #[test]
    fn test_get_order_topological_sort_2() {
        assert_eq!(
            get_order_topological_sort(4, &[(1, 0), (2, 0), (3, 1), (3, 2)]),
            vec![0, 1, 2, 3]
        );
    }

    #[test]
    fn test_get_order_topological_sort_3() {
        assert_eq!(get_order_topological_sort(1, &[]), vec![0]);
    }

    #[test]
    fn test_get_order_topological_sort_4() {
        assert_eq!(get_order_topological_sort(2, &[(1, 0), (0, 1)]), vec![]);
    }
}
