// https://leetcode.com/problems/course-schedule-ii/solutions/756241/rust-cheapest-best/
impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let num_courses = num_courses as usize;
        let mut num_requirements = vec![0; num_courses];
        let mut children = vec![Vec::with_capacity(num_courses); num_courses];
        let mut result = Vec::with_capacity(num_courses);

        prerequisites.iter().for_each(|req| {
            children[req[1] as usize].push(req[0]);
            num_requirements[req[0] as usize] += 1;
        });

        let mut stack = vec![];
        for i in 0..num_courses {
            if num_requirements[i] == 0 {
                stack.push(i);
            }
        }

        while stack.len() > 0 {
            let course = stack.pop().unwrap();
            result.push(course as i32);
            for n in &children[course] {
                num_requirements[*n as usize] -= 1;
                if num_requirements[*n as usize] == 0 {
                    stack.push(*n as usize);
                }
            }
        }

        if result.len() == num_courses { result } else { vec![] }
    }
}