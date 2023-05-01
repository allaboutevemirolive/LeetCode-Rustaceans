// https://leetcode.com/problems/course-schedule/solutions/923758/rust-iterative-post-order-dfs-detect-back-edge/
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let num_courses = num_courses as usize;
        let mut edges = vec![vec![]; num_courses];
        for edge in prerequisites.iter() {
            edges[edge[0] as usize].push(edge[1] as usize);
        }
        let mut visited = vec![false; num_courses];
        let mut post_orderz = vec![0; num_courses];
        let mut post_order = 0;
        
        for i in 0..num_courses {
            if !visited[i] {
                visited[i] = true;
                
                let mut stack = vec![i];
                
                while !stack.is_empty() {
                    let cur = stack[stack.len() - 1];
                    
                    let mut ready_to_pop = true;
                    for &successor in edges[cur].iter() {
                        if !visited[successor] {
                            ready_to_pop = false;
                            visited[successor] = true;
                            stack.push(successor);
                            break;
                        }
                    }
                    
                    // pop only when there are no more children to be processed
                    if ready_to_pop {
                        post_orderz[stack[stack.len() - 1]] = post_order;
                        stack.pop();
                        post_order += 1;
                    }
                }
            }
        }
        
        // If the forest contains a back-edge,
        // there is a cyclic dependency between courses.
        for i in 0..num_courses {
            for &successor in edges[i].iter() {
                if post_orderz[i] < post_orderz[successor] {
                    return false;
                }   
            }
        }
        
        return true;
    }
}