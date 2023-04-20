// https://leetcode.com/problems/cut-off-trees-for-golf-event/solutions/875109/rust-bfs/
impl Solution {
    const DR: [i32; 4] = [-1, 1, 0, 0];
const DC: [i32; 4] = [0, 0, -1, 1];
    pub fn cut_off_tree(forest: Vec<Vec<i32>>) -> i32 {
        let mut trees: Vec<Vec<i32>> = Vec::new();
    for i in 0..forest.len() {
        for j in 0..forest[0].len() {
            let v = forest[i][j];
            if v > 1 {
                trees.push(vec![v, i as i32, j as i32]);
            }
        }
    }
    trees.sort();
    let mut ans = 0;
    let mut sr = 0;
    let mut sc = 0;
    for tree in trees.iter() {
        let d = Self::bfs(&forest, sr, sc, tree[1], tree[2]);
         if d < 0 {
            return -1;
        }
        ans += d;
        
        sr = tree[1];
        sc = tree[2];
    }
    ans
    }
    fn bfs(forest: &Vec<Vec<i32>>, sr: i32, sc: i32, tr: i32, tc: i32) -> i32 {
    let fr = forest.len();
    let fc = forest[0].len();
    let mut queue: std::collections::VecDeque<Vec<i32>> = std::collections::VecDeque::new();
    queue.push_back(vec![sr, sc, 0]);
    let mut seen: Vec<Vec<bool>> = vec![vec![false; fc]; fr];
    seen[sr as usize][sc as usize] = true;
    while !queue.is_empty() {
        let cur = queue.pop_front().unwrap();
        if cur[0] == tr && cur[1] == tc {
            return cur[2];
        }
        for di in 0..4 {
            let r = (cur[0] + Self::DR[di]) as usize;
            let c = (cur[1] + Self::DC[di]) as usize;
            if 0 <= r && r < fr && 0 <= c && c < fc && !seen[r][c] && forest[r][c] > 0 {
                seen[r][c] = true;
                queue.push_back(vec![r as i32, c as i32, cur[2] + 1]);
            }
        }
    }
    -1
}
}