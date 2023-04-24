// https://leetcode.com/problems/escape-a-large-maze/solutions/904126/rust-simple-floodfill-20-ms/
use std::collections::HashSet;
use std::convert::TryFrom;

fn gen_neighbors((x, y): (u32, u32))->[Option<(u32, u32)>;4]{
    const LIMIT: u32 = 1000_000;
    [
        if x>0 {Some((x-1, y))} else {None},
        if x+1 < LIMIT {Some((x+1, y))} else {None},
        if y>0 {Some((x, y-1))} else {None},
        if y+1 < LIMIT {Some((x, y+1))} else {None},
    ]
}

#[derive(Eq, PartialEq)]
enum States{
    Possible,
    Blocked,
    Solved,
}

fn try_flood_fill(pos: (u32, u32), target: (u32, u32), blocked: &HashSet<(u32, u32)>, block_rows: &HashSet<u32>, block_cols:&HashSet<u32>)->States{
    // if we can flood at least 6xnumber of blocked tiles
    let mut flooded = HashSet::new();
    let mut stack = vec![pos];
    while let Some(curr) = stack.pop(){
        if curr == target{
            return States::Solved;
        }
        if !block_rows.contains(&curr.0){
            return States::Possible;
        }
        if !block_cols.contains(&curr.1){
            return States::Possible;
        }
        flooded.insert(curr);
        for neighbor in gen_neighbors(curr).iter().copied().flatten(){
            if !blocked.contains(&neighbor) && !flooded.contains(&neighbor){
                stack.push(neighbor);
            }
        }
    }
    States::Blocked
}

fn conv_vec(v: Vec<i32>)->(u32, u32){
    (u32::try_from(v[0]).unwrap(), u32::try_from(v[1]).unwrap())
}

impl Solution {
    pub fn is_escape_possible(blocked: Vec<Vec<i32>>, source: Vec<i32>, target: Vec<i32>) -> bool {       
        if blocked.is_empty(){
            return true;
        }
        
        let source = conv_vec(source);
        let target = conv_vec(target);
        let blocked: HashSet<(u32, u32)> = blocked.into_iter()
            .map(conv_vec)
            .collect();
        let block_rows: HashSet<u32> = blocked.iter().copied().map(|(x,_)|x).collect();
        let block_cols: HashSet<u32> = blocked.iter().copied().map(|(_,y)|y).collect();
        
        let r1 = try_flood_fill(source, target, &blocked, &block_rows, &block_cols);
        if r1==States::Solved{
            return true;
        }
        let r2 = try_flood_fill(target, source, &blocked, &block_rows, &block_cols);
        match (r1, r2){
            (States::Possible, States::Solved) => true,
            (States::Possible, States::Possible) => true,
            _ => false,
        }
    }
}