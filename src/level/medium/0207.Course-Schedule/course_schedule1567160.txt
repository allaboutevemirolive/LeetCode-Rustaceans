// https://leetcode.com/problems/course-schedule/solutions/1567160/rust-0ms-2-3mb-topological-sort-w-explanation/
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut num_courses = num_courses as usize;
        let mut catalog: Vec<Vec<usize>> = vec![vec![]; num_courses];
        
        //  the number of courses you need to take before you can take prereq[i]
        let mut prereq: Vec<i32> = vec![0; num_courses];
        let mut res: Vec<usize> = Vec::new();
        
        for edge in prerequisites {
        
			//  b -> a
            let a = edge[0] as usize;
            let b = edge[1] as usize;
            
            //  For example, the pair, || [a, b]
            //  indicates that to take course a you have to first take course b.
            catalog[b].push(a);
            
			// prereq table: 
            //  a: 1
            //  b: b 
            prereq[a] += 1;
            
        }
        
        //  Available courses || For example, available: a, b, e
        let mut available = vec![];
        
        for i in 0..num_courses { 
            //  if there are no prereqs required here
            if prereq[i] == 0 { 
                //  push into available list
                available.push(i)
            }
        }
        //  Simulate taking courses 
        while let Some(course) =  available.pop() { 
            res.push(course);
            //  available: a.pop,  e ||  res: a
            
            //  Simulate 
            //  After popping 'a', means we can take 'b' now
            while let Some(depend) =  catalog[course].pop() { 
                prereq[depend] -= 1;
                //  'b' now has -1 dependecies
                
                //  if 'b' has zero dependencies
                if prereq[depend] == 0 { 
                    available.push(depend)
                }
            }
        }
        //  Can we finish all courses? 
        //  if res == original number of courses then yes 
        res.len() == num_courses 
    }
}