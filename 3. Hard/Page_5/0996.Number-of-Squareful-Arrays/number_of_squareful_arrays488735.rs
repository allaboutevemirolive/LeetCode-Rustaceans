// https://leetcode.com/problems/number-of-squareful-arrays/solutions/488735/rust-44ms-2-3mb-100-translated/
use std::collections::HashMap;

pub fn num_squareful_perms(a: Vec<i32>) -> i32 {
    let n = a.len();
    let mut graph = HashMap::<i32, Vec<i32>>::new();
    let mut memo = Vec::<Vec<i32>>::new();
    for _ in 0..n { memo.push(vec![i32::min_value(); 1 << n]) }
    for i in 0..n {
        graph.insert(i as i32, Vec::<i32>::new());
    }
    for i in 0..n {
        for j in i + 1..n {
            let f = ((a[i] + a[j]) as f64).sqrt() + 0.5;
            let r = f as i32;
            if r * r == a[i] + a[j] {
                graph.entry(i as i32).or_default().push(j as i32);
                graph.entry(j as i32).or_default().push(i as i32);
            }
        }
    }

    let mut factorial = [0; 20];
    factorial[0] = 1;
    for i in 1..20 {
        factorial[i] = i * factorial[i - 1];
    }
    let mut ans = 0;

    for i in 0..n { ans += dfs(n as i32, &mut graph, &mut memo, i as i32, (1 << i) as i32); }

    let mut count = HashMap::<i32, i32>::new();
    for x in a {
        *count.entry(x).or_default() += 1;
    }
    for v in count.values() { ans /= factorial[*v as usize] as i32; }
    return ans;


    fn dfs(n: i32, graph: &mut HashMap<i32, Vec<i32>>, memo: &mut Vec<Vec<i32>>, node: i32, visited: i32) -> i32 {
        if visited == (1 << n) - 1 { return 1; }
        if memo[node as usize][visited as usize] != i32::min_value() {
            return memo[node as usize][visited as usize];
        }
        let mut ans = 0;
        if let Some(x) = graph.clone().get(&node) {
            for nei in x {
                if ((visited >> *nei) & 1) == 0 {
                    ans += dfs(n, graph, memo, *nei, visited | (1 << *nei));
                }
            }
        }
        memo[node as usize][visited as usize] = ans;
        ans
    }
}
