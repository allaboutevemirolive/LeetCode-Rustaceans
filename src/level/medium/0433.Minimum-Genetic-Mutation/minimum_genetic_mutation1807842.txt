// https://leetcode.com/problems/minimum-genetic-mutation/solutions/1807842/rust-bi-directional-bfs-with-word-list-pruning/
use std::collections::HashSet;

const NO_SOLUTION: i32 = -1;

pub fn min_mutation(begin_word: String, end_word: String, mut word_list: Vec<String>) -> i32 {
    match word_list.iter().position(|w| end_word.eq(w)) {
        None => return NO_SOLUTION,
        Some(idx) => word_list.swap_remove(idx),
    };

    let mut left = HashSet::new();
    let mut right = HashSet::new();
    let mut next = HashSet::new();

    left.insert(begin_word);
    right.insert(end_word);

    let mut len = 0;
    while !left.is_empty() && !right.is_empty() {
        if left.len() > right.len() {
            std::mem::swap(&mut left, &mut right);
        }

        for w in left.drain() {
            let mut idx = 0;

            for word in right.iter() {
                if is_adjacent(&w, word.as_str()) {
                    return len + 1;
                }
            }

            while idx < word_list.len() {
                if !is_adjacent(&w, &word_list[idx]) {
                    idx += 1;
                    continue;
                }

                let neighbour = word_list.swap_remove(idx);
                next.insert(neighbour);
            }
        }

        std::mem::swap(&mut left, &mut next);
        len += 1;
    }

    NO_SOLUTION
}

fn is_adjacent(a: &str, b: &str) -> bool {
    let a = a.as_bytes();
    let b = b.as_bytes();
    assert_eq!(a.len(), b.len());

    let mut diffs = 0;
    for idx in 0..a.len() {
        if a[idx] != b[idx] {
            diffs += 1;
        }

        if diffs > 1 {
            break;
        }
    }

    diffs == 1
}