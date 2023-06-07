// https://leetcode.com/problems/redundant-connection-ii/solutions/494935/rust-0ms-2mb-100/
pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    fn find(ds: &mut Vec<i32>, i: i32) -> i32 {
        if ds[i as usize] == 0 { i } else {
            ds[i as usize] = find(ds, ds[i as usize]);
            ds[i as usize]
        }
    }
    let n = edges.len();
    let mut parents = vec![-1; n + 1];
    let mut ds = vec![0; n + 1];
    let mut x1 = -1i32;
    let mut x2 = -1i32;
    let mut xn = -1i32;
    for i in 0..n {
        let p = edges[i][0];
        let c = edges[i][1];
        if parents[c as usize] != -1 {
            x1 = parents[c as usize];
            x2 = i as i32;
            continue;
        }
        parents[c as usize] = i as i32;
        let p1 = find(&mut ds, p);
        if p1 == c { xn = i as i32; } else { ds[c as usize] = p1; }
    }
    if xn == -1 { return edges[x2 as usize].clone(); }
    if x2 == -1 { return edges[xn as usize].clone(); }
    edges[x1 as usize].clone()
}
