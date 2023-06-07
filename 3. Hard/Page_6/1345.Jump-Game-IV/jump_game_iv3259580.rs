// https://leetcode.com/problems/jump-game-iv/solutions/3259580/simple-bfs-solution-in-rust/
impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut shortcuts = arr.iter().copied().enumerate().fold(std::collections::HashMap::new(),
            |mut map, (i, val)| { map.entry(val).or_insert(vec![]).push(i); map });
        let mut visited = vec![false; arr.len()];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(0);
        queue.push_back(usize::MAX);
        visited[0] = true;
        let mut jump_count = 0;
        while let Some(node) = queue.pop_front() {
            if node==n-1 {
                break;
            }
            if node==usize::MAX {
                jump_count += 1;
                queue.push_back(usize::MAX);
            }
            else {
                let min = node.checked_sub(1).unwrap_or(0);
                let max = node + 1;
                for adj in shortcuts.remove(&arr[node]).into_iter().flatten().chain(min..=max).filter(|&adj| adj!=node) {
                    if !visited[adj] {
                        visited[adj] = true;
                        queue.push_back(adj);
                    }
                }
            }
        }
        jump_count
    }
}