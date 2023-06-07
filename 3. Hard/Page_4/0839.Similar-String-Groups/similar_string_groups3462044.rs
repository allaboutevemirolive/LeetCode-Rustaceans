// https://leetcode.com/problems/similar-string-groups/solutions/3462044/bfs-solution-rust/
impl Solution {
    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        let mut ans = 0;
        let n = strs.len();
        let mut visited = vec![false; n];
        let mut adj = vec![Vec::new(); n];

       for i in 0..n {
            for j in i+1..n {
                if is_similar(&strs[i], &strs[j]) {
                    adj[i].push(j);
                    adj[j].push(i);
                }
            }
        }
        
        for i in 0..n {
            if !visited[i] {
                ans += 1;
                visited[i] = true;
                bfs(i, &adj, &mut visited);
            }
        }

        ans
    }
}

fn is_similar(s1: &String, s2: &String) -> bool {
    let mut diff = 0;
    for i in 0..s1.len() {
        diff += (&s1[i..=i] != &s2[i..=i]) as i32;
        if diff > 2 { return false };
    }

    diff != 1
}

fn bfs(mut node: usize, adj: &Vec<Vec<usize>>, visited: &mut Vec<bool>) {
    let mut queue = vec![node];

    while queue.len() > 0 {
        node = queue.pop().unwrap();

        for &next in adj[node].iter() {
            if !visited[next] {
                queue.push(next);
                visited[next] = true;
            }
        }
    }
}