// https://leetcode.com/problems/stone-game-iii/solutions/3567341/rust-o-n-with-queue/
use std::cmp::Ordering;
use std::collections::VecDeque;

impl Solution {
    pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
        let mut queue = VecDeque::from(vec![0;3]);
        let mut acc = 0;
        for stone in stone_value.into_iter().rev() {
            acc += stone;
            let next = acc - queue.iter().min().unwrap();
            queue.pop_front();
            queue.push_back(next);
        }
        let alice = queue.pop_back().unwrap();
        let bob = acc - alice;
        match alice.cmp(&bob) {
            Ordering::Less => "Bob",
            Ordering::Equal => "Tie",
            Ordering::Greater => "Alice",
        }.to_string()
    }
}