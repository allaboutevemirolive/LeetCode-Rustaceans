// https://leetcode.com/problems/trapping-rain-water-ii/solutions/506561/rust-priority-queue-solution/
use std::cmp::Ordering;

#[derive(Eq)]
struct Cell{
    pub row:usize,
    pub col:usize,
    pub height:i32,       
}

impl Cell {
    fn new(row:usize,col:usize,height:i32)->Self{
        Cell{
            row,
            col,
            height
        }
    }
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        self.height.cmp(&other.height)
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Cell{
    fn eq(&self, other: &Self) -> bool {
        self.height == other.height
    }
}

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        use std::collections::BinaryHeap;
        use std::cmp::Reverse;
        let m = height_map.len();
        if m == 0 {
            return 0;
        }
        let n = height_map[0].len();
        if n==0 {
            return 0;
        }
        let mut queue = BinaryHeap::new();
        let mut visited = vec![vec![false;n];m];
        //visit the borders
        for i in 0..m {
            queue.push(Reverse(Cell::new(i,0,height_map[i][0])));
            queue.push(Reverse(Cell::new(i,n-1,height_map[i][n-1])));
            visited[i][0]=true;
            visited[i][n-1]=true;
        }
        for j in 0..n{
            queue.push(Reverse(Cell::new(0,j,height_map[0][j])));
            queue.push(Reverse(Cell::new(m-1,j,height_map[m-1][j])));
            visited[0][j]=true;
            visited[m-1][j]=true;
        }
        //top down left right
        let dirs = [(0,-1),(0,1),(-1,0),(1,0)];
        let mut ans = 0;
        while !queue.is_empty(){
            if let Some(Reverse(curr)) = queue.pop(){
                for dir in &dirs {
                    let r = curr.row as i32 + dir.0;
                    let c = curr.col as i32 + dir.1;
                    if r >0 && r <m as i32 -1 && c>0 && c< n as i32-1 && !visited[r as usize][c as usize]{
                        let r = r as usize;
                        let c = c as usize;
                        ans += 0.max(curr.height - height_map[r][c]);
                        queue.push(Reverse(Cell::new(r,c,height_map[r][c].max(curr.height))));
                        visited[r][c] = true;
                    }
                }
            }
        }
        ans
    }
}