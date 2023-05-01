// https://leetcode.com/problems/course-schedule-ii/solutions/1570855/rust-0ms-and-2-3-mb-0ms-and-95-mem-leetcode-course-schedule-i-vecdeque/
use std::collections::VecDeque;
impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut num_courses = num_courses as usize;
        let mut catalog: Vec<Vec<usize>> = vec![vec![]; num_courses];
        let mut prereq: Vec<i32> = vec![0; num_courses];
        let mut res = Vec::with_capacity(num_courses);        
        
        for edge in prerequisites { 
            let a = edge[0] as usize;
            let b = edge[1] as usize;
            catalog[b].push(a);
            prereq[a] += 1
        }
        
        let mut available = VecDeque::with_capacity(num_courses);
        for i in 0..num_courses { 
            if prereq[i] == 0 { 
                available.push_back(i);
            }
        }
        
        while let Some(finished_course) = available.pop_front() { 
            res.push(finished_course as i32);
            while let Some(new) = catalog[finished_course].pop() { 
                prereq[new] -= 1;
                if prereq[new] == 0 {
                    available.push_back(new);
                } 
            } 
        }
        
        if res.len() == num_courses {  
            res
        } else { 
            vec![]
        }
    }
}
