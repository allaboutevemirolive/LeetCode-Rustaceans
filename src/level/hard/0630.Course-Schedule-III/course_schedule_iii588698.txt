// https://leetcode.com/problems/course-schedule-iii/solutions/588698/rust-binaryheap-28ms/
use std::collections::BinaryHeap;

impl Solution {
    pub fn schedule_course(mut courses: Vec<Vec<i32>>) -> i32 {
        courses.sort_by(|a, b| a[1].cmp(&b[1]));
        let mut heap: BinaryHeap<i32> = BinaryHeap::new();
        let mut time = 0;
        for course in courses.into_iter() {
            if time + course[0] <= course[1] {
                time += course[0];
                heap.push(course[0]);
            } else {
                if let Some(&previous_time) = heap.peek() {
                    if previous_time > course[0] {
                        time -= previous_time - course[0];
                        heap.pop();
                        heap.push(course[0]);
                    }
                }
            }
        }
        heap.len() as i32
    }
}