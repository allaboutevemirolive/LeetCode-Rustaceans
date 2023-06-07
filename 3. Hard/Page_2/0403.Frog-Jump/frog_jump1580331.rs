// https://leetcode.com/problems/frog-jump/solutions/1580331/rust-8ms-100-solution-pre-populated-map-w-dp-recursive/
use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Eq, PartialEq, Hash, Clone)]
struct FrogState {
    k: i32,
    p: i32
}

fn can_jump(k: i32, position: i32, stones: &HashSet<i32>, target: i32, memo: &mut HashSet<FrogState>) -> bool {
    // println!("K {:?}", &k);
    // println!("Position {:?}", &position);
    // println!("Stones {:?}", &stones);
    // println!("Target {:?}", &target);
    // println!("-------");
    let state = FrogState {
        k: k,
        p: position
    };
    
    if memo.contains(&state) {
        return false
    }

    if !stones.contains(&position) {
        return false
    }
    if position == target {
        return true
    }
    if position > target {
        return false
    }
    if k < 1 {
        return false
    }
    if can_jump(k-1, position+k-1, &stones, target, memo) || can_jump(k, position + k, &stones, target, memo) || can_jump(k+1, position + k + 1, &stones, target, memo) {
        return true
    }
    memo.insert(state);
    false
    

}

fn can_jump_first(stones: &HashSet<i32>, target: i32) -> bool {
    can_jump(1, 1, &stones, target, &mut HashSet::new())
}

impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        let mut stones_map = HashSet::from_iter(stones.iter().cloned());
        let target_pos = stones.last().copied().unwrap();
        can_jump_first(&stones_map, target_pos)
    }
}