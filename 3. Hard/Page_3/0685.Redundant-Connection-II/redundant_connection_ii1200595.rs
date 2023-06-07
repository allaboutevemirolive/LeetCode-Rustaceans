// https://leetcode.com/problems/redundant-connection-ii/solutions/1200595/rust-union-find-with-comments/
impl Solution {
    pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut indegrees = vec![0; n + 1];
        let mut node_with_indegree_of_2 = None;
        for edge in &edges {
            let target = edge[1] as usize;
            if indegrees[target] == 1 {
                node_with_indegree_of_2 = Some(target);
                break;
            }
            indegrees[target] += 1;
        }

        if let Some(node) = node_with_indegree_of_2 {
            // The added edge is directed to a non-root node. It must be one of the two edges
            // that are directed to that node.
            let mut candidates = vec![];
            let mut uf = UnionFind::new(n + 1);
            for edge in &edges {
                let (source, target) = (edge[0] as usize, edge[1] as usize);
                if target == node {
                    candidates.push(source);
                } else {
                    uf.union(source, target);
                }
            }
            // If the first candidate doesn't cause a cycle, we could happily remove the second candidate.
            // Otherwise we have to remove the first one.
            if uf.union(candidates[0], node) {
                vec![candidates[1] as i32, node as i32]
            } else {
                vec![candidates[0] as i32, node as i32]
            }
        } else {
            // The added edge is directed to the root. Remove any edge in the cycle would work.
            // We prefer the one occurs last.
            let mut uf = UnionFind::new(n + 1);
            for edge in edges {
                let (source, target) = (edge[0] as usize, edge[1] as usize);
                if !uf.union(source, target) {
                    return edge;
                }
            }
            unreachable!("");
        }
    }
}

struct UnionFind {
    parents: Vec<usize>,
    sizes: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            parents: (0..size).collect(),
            sizes: vec![1; size],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        let mut root = x;
        let Self { parents, .. } = self;
        while parents[root] != root {
            // Path splitting
            let parent = parents[root];
            parents[root] = parents[parent];
            root = parent;
        }
        root
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let mut root1 = self.find(x);
        let mut root2 = self.find(y);
        if root1 == root2 {
            return false;
        }

        // Union by size
        let Self { parents, sizes } = self;
        if sizes[root1] > sizes[root2] {
            std::mem::swap(&mut root1, &mut root2);
        }
        parents[root1] = root2;
        sizes[root2] += sizes[root1];
        true
    }
}