// https://leetcode.com/problems/erect-the-fence/solutions/792325/rust-translated-8ms-2-2m-100/
pub fn outer_trees(points: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    use std::collections::HashSet;

    fn orientation(p: &[i32], q: &[i32], r: &[i32]) -> i32 {
        (q[1] - p[1]) * (r[0] - q[0]) - (q[0] - p[0]) * (r[1] - q[1])
    }

    let mut points = points;
    if points.len() <= 1 {
        return points;
    }
    points.sort_by(|p, q| p[0].cmp(&q[0]).then(p[1].cmp(&q[1])));

    let mut hull = Vec::<Vec<i32>>::new();
    let n = points.len();
    for i in 0..n {
        while hull.len() >= 2
            && orientation(
                hull.get(hull.len() - 2).unwrap(),
                hull.get(hull.len() - 1).unwrap(),
                &points[i],
            ) > 0
        {
            hull.pop();
        }
        hull.push(points[i].clone());
    }
    for i in (0..n).rev() {
        while hull.len() >= 2
            && orientation(
                hull.get(hull.len() - 2).unwrap(),
                hull.get(hull.len() - 1).unwrap(),
                &points[i],
            ) > 0
        {
            hull.pop();
        }
        hull.push(points[i].clone());
    }
    let mut set = HashSet::<Vec<i32>>::new();
    for v in hull {
        set.insert(v);
    }
    let mut ans = vec![];
    for v in set {
        ans.push(v);
    }
    ans
}