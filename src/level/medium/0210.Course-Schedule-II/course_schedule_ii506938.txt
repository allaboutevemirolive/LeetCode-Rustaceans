// https://leetcode.com/problems/course-schedule-ii/solutions/506938/rust-topological-sort-solution/
impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut v: Vec<(usize, Vec<usize>)> = vec![(0, vec![]); num_courses as usize];
        for p in prerequisites.iter() {
            v[p[0] as usize].0 += 1;
            v[p[1] as usize].1.push(p[0] as usize);
        }
        let mut stack: Vec<usize> = Vec::new();
        for (i, e) in (0..).zip(v.iter()) {
            if e.0 == 0 {
                stack.push(i);
            }
        }
        let mut answer: Vec<i32> = Vec::with_capacity(num_courses as usize);
        while let Some(last) = stack.pop() {
            answer.push(last as i32);
            for i in v[last].1.clone() {
                v[i].0 -= 1;
                if v[i].0 == 0 {
                    stack.push(i);
                }
            }
        }
        if answer.len() == num_courses as usize {
            answer
        } else {
            vec![]
        }
    }
}