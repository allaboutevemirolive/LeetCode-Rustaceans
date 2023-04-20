// https://leetcode.com/problems/contain-virus/solutions/892024/rust-translated-8ms-100/
impl Solution {
    pub fn contain_virus(mut grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashSet;

        const DIRS: [i32; 5] = [0, -1, 0, 1, 0];

        fn dfs(
            grid: &mut Vec<Vec<i32>>,
            regions: &mut Vec<HashSet<i32>>,
            frontiers: &mut Vec<HashSet<i32>>,
            perimeters: &mut Vec<i32>,
            visited: &mut HashSet<i32>,
            r: i32,
            c: i32,
        ) {
            let m = grid.len();
            let n = grid[0].len();
            if !visited.contains(&(r * n as i32 + c as i32)) {
                visited.insert(r * n as i32 + c as i32);
            }
            let n2 = regions.len();
            regions[n2 - 1].insert(r * n as i32 + c as i32);
            for k in 0..4 {
                let nr = r + DIRS[k];
                let nc = c + DIRS[k + 1];
                if nr >= 0 && nr < m as i32 && nc >= 0 && nc < n as i32 {
                    if grid[nr as usize][nc as usize] == 1
                        && !visited.contains(&(nr * n as i32 + nc as i32))
                    {
                        dfs(grid, regions, frontiers, perimeters, visited, nr, nc);
                    } else if grid[nr as usize][nc as usize] == 0 {
                        frontiers[n2 - 1].insert(nr * n as i32 + nc);
                        perimeters[n2 - 1] += 1;
                    }
                }
            }
        }

        let mut visited = HashSet::<i32>::new();
        let mut regions = Vec::<HashSet<i32>>::new();
        let mut frontiers = Vec::<HashSet<i32>>::new();
        let mut perimeters = Vec::<i32>::new();
        let m = grid.len();
        let n = grid[0].len();
        let mut ans = 0;
        loop {
            visited.clear();
            regions.clear();
            frontiers.clear();
            perimeters.clear();
            for r in 0..m {
                for c in 0..n {
                    if grid[r][c] == 1 && !visited.contains(&((r * n + c) as i32)) {
                        regions.push(HashSet::new());
                        frontiers.push(HashSet::new());
                        perimeters.push(0);
                        dfs(
                            &mut grid,
                            &mut regions,
                            &mut frontiers,
                            &mut perimeters,
                            &mut visited,
                            r as i32,
                            c as i32,
                        );
                    }
                }
            }
            if regions.is_empty() {
                break;
            }

            let mut triage_index = 0;
            for i in 0..frontiers.len() {
                if frontiers[triage_index].len() < frontiers[i].len() {
                    triage_index = i;
                }
            }

            ans += perimeters.get(triage_index).unwrap_or_else(|| &0);

            for i in 0..regions.len() {
                if i == triage_index {
                    for &code in &regions[i] {
                        grid[code as usize / n][code as usize % n] = -1;
                    }
                } else {
                    for &code in &regions[i] {
                        let r = code / n as i32;
                        let c = code % n as i32;
                        for k in 0..4 {
                            let nr = r + DIRS[k];
                            let nc = c + DIRS[k + 1];
                            if nr >= 0
                                && nr < m as i32
                                && nc >= 0
                                && nc < n as i32
                                && grid[nr as usize][nc as usize] == 0
                            {
                                grid[nr as usize][nc as usize] = 1;
                            }
                        }
                    }
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contain_virus() {
        assert_eq!(
            Solution::contain_virus(vec![
                vec![0, 1, 0, 0, 0, 0, 0, 1],
                vec![0, 1, 0, 0, 0, 0, 0, 1],
                vec![0, 0, 0, 0, 0, 0, 0, 1],
                vec![0, 0, 0, 0, 0, 0, 0, 0]
            ]),
            10
        );
    }

    #[test]
    fn test_contain_virus_02() {
        assert_eq!(
            Solution::contain_virus(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]),
            4
        );
    }

    #[test]
    fn test_contain_virus_03() {
        assert_eq!(
            Solution::contain_virus(vec![
                vec![1, 1, 1, 0, 0, 0, 0, 0, 0],
                vec![1, 0, 1, 0, 1, 1, 1, 1, 1],
                vec![1, 1, 1, 0, 0, 0, 0, 0, 0]
            ]),
            13
        );
    }
}