// https://leetcode.com/problems/jump-game-iv/solutions/1692404/rust-bfs/
/// @author: Leon
/// https://leetcode.com/problems/jump-game-iv/
/// Time Complexity:    O(V + E)
/// Space Complexity:   O(V + E) 
/// Reference:
/// https://leetcode.com/problems/jump-game-iv/discuss/502699/JavaC%2B%2B-BFS-Solution-Clean-code-O(N)
use std::collections::{HashMap, HashSet, VecDeque};
impl Solution {
    pub fn min_jumps(nums: Vec<i32>) -> i32 {
        let len_n: usize = nums.len();
        if len_n == 1 {
            return 0;
        }
        let mut graph = Self::build_graph(&nums);
        let mut queue: VecDeque<usize> = VecDeque::new();
        queue.push_back(0);
        let mut seen: HashSet<usize> = HashSet::new();
        seen.insert(0);
        let mut steps: i32 = 0;
        while !queue.is_empty() {
            let len_q: usize = queue.len();
            for _ in 0..len_q {
                if let Some(cur) = queue.pop_front() {
                    if cur == len_n - 1 {
                        return steps;
                    }
                    if let Some(nxts) = graph.get_mut(&nums[cur]) {
                        if cur >= 1 {
                            nxts.push(cur - 1);
                        }
                        if cur + 1 < len_n {
                            nxts.push(cur + 1);
                        }
                        while let Some(nxt) = nxts.pop() {
                            if seen.insert(nxt) {
                                queue.push_back(nxt);
                            }
                        }
                    };
                }
            }
            steps += 1;
        }
        steps
    }
    fn build_graph(nums: &Vec<i32>) -> HashMap<i32, Vec<usize>> {
        let mut graph: HashMap<i32, Vec<usize>> = HashMap::new();
        for (idx, &num) in nums.iter().enumerate() {
            graph.entry(num).or_default().push(idx);
        }
        graph
    }
}