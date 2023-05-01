// https://leetcode.com/problems/course-schedule-ii/solutions/1643685/rust-kahn-s-algorithm/
/// @author: Leon
/// https://leetcode.com/problems/course-schedule-ii/
/// Time Complexity:    O(`num_courses` + `_n_pres`)
/// Space Complexity:   O(`num_courses` + `n_pres`)
use std::collections::VecDeque;
impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let (graph, mut indegrees) = Self::build_graph(num_courses, prerequisites);
        let mut queue: VecDeque<usize> = VecDeque::new();
        for (idx, &indegree) in indegrees.iter().enumerate(){
            if indegree == 0{
                queue.push_back(idx);
            }
        }
        let mut idx: usize = 0;
        let ans: Vec<i32> = {
            let mut ans = vec![0; num_courses as usize];
            while !queue.is_empty(){
                let cur = queue.pop_front().unwrap();
                ans[idx] = cur as i32;
                idx += 1;
                for &prev in graph[cur].iter(){
                    indegrees[prev as usize] -= 1;
                    if indegrees[prev as usize] == 0{
                        queue.push_back(prev as usize);
                    }
                }
            }
            ans
        };
        if idx < num_courses as usize{
            return vec![];
        }
        ans
    }
    fn build_graph(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> (Vec<Vec<i32>>, Vec<i32>){
        let mut indegrees: Vec<i32> = vec![0; num_courses as usize];
        let mut graph: Vec<Vec<i32>> = vec![vec![]; num_courses as usize];
        for pres in prerequisites{
            let cur = pres[0];
            let pre = pres[1];
            graph[pre as usize].push(cur);
            indegrees[cur as usize] += 1;
        }
        (graph, indegrees)
    }
}