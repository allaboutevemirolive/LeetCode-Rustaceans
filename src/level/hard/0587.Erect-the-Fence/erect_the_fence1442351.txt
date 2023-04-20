// https://leetcode.com/problems/erect-the-fence/solutions/1442351/rust-graham/
fn turn(a: &[i32], b: &[i32], c: &[i32]) -> i32 {
    (c[0] - a[0]) * (b[1] - a[1]) - (c[1] - a[1]) * (b[0] - a[0])
}

impl Solution {
    pub fn outer_trees(trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut a = trees;
        {
            let pos_min = a
                .iter()
                .enumerate()
                .min_by_key(|(_, x)| (x[0], x[1]))
                .unwrap()
                .0;
            a.swap(0, pos_min);
            let (f, rest) = a.split_first_mut().unwrap();
            rest.sort_unstable_by(|a, b| (turn(f, a, b), b).cmp(&(0, a)));
        }
        if let Some(f) = a
            .get(2..)
            .and_then(|t| t.iter().position(|x| turn(&a[0], &a[1], x) != 0))
        {
            let mut cnt = 2 + f;
            a[1..cnt].reverse();
            for i in cnt..a.len() {
                while cnt >= 1 && turn(&a[cnt - 2], &a[cnt - 1], &a[i]) > 0 {
                    cnt -= 1;
                }
                a[cnt] = a[i].clone();
                cnt += 1;
            }
            a.resize_with(cnt, || unreachable!());
        }
        a
    }
}