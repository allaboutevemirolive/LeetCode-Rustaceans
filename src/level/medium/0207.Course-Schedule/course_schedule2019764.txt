// https://leetcode.com/problems/course-schedule/solutions/2019764/rust-dfs-with-coloring/
#[derive(PartialEq, Clone)]
enum Status {
    Waiting,
    Inprogress,
    Finished
}

fn dfs(graph: &Vec<Vec<usize>>, status: &mut Vec<Status>, node: usize) -> bool {
    status[node] = Status::Inprogress;
    for &child in graph[node].iter() {
        if status[child] == Status::Inprogress {
            return false;
        }
        if status[child] == Status::Finished {
            continue;
        }
        if !dfs(graph, status, child) {
            return false;
        }
    }
    status[node] = Status::Finished;
    true
}

pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let n = num_courses as usize;
    let mut graph = vec![Vec::with_capacity(n); n];
    for prereq in prerequisites {
        graph[prereq[1] as usize].push(prereq[0] as usize);
    }

    let mut status = vec![Status::Waiting;n];
    for i in 0..n {
        if status[i] == Status::Finished {
            continue;
        }
        
        if !dfs(&graph, &mut status, i) {
            return false;
        }
    }
    true
}