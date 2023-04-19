// https://leetcode.com/problems/freedom-trail/solutions/2418385/rust-dijkstra/
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

const INITIAL_DIAL_POSITION: usize = 0;
const FIRST_KEY_CHARACTER: usize = 0;

pub fn find_rotate_steps(ring: impl AsRef<str>, key: impl AsRef<str>) -> i32 {
    let ring = ring.as_ref();
    let key = key.as_ref().as_bytes();
    let rlen = ring.len();

    let positions = build_ring_char_positions(ring);
    let mut pq = BinaryHeap::new();

    // Seed the Dijkstra algorithm with the first character of the key
    if let Some(indexes) = positions.get(&key[FIRST_KEY_CHARACTER]) {
        for ring_idx in indexes.iter().copied() {
            pq.push((
                Reverse(dial_cost(rlen, INITIAL_DIAL_POSITION, ring_idx)),
                ring_idx,
                FIRST_KEY_CHARACTER,
            ));
        }
    }

    let mut visited = vec![vec![i32::MAX; key.len()]; ring.len()];

    while let Some((Reverse(cost), ring_idx, key_idx)) = pq.pop() {
        // We have "entered" the whole key, so we can stop here
        if key_idx == key.len() - 1 {
            // Add the key length to the cost, because of the "dial" button
            return cost + key.len() as i32;
        }

        // Skip already visited nodes with better cost
        if visited[ring_idx][key_idx] <= cost {
            continue;
        }
        visited[ring_idx][key_idx] = cost;

        if let Some(indexes) = positions.get(&key[key_idx + 1]) {
            for new_ring_idx in indexes.iter().copied() {
                let new_cost = cost + dial_cost(rlen, ring_idx, new_ring_idx);

                if visited[new_ring_idx][key_idx + 1] > new_cost {
                    pq.push((Reverse(new_cost), new_ring_idx, key_idx + 1));
                }
            }
        }
    }

    //The problem statement guarantees us that the "key" can be entered
    unreachable!()
}

// This function calculates the minimum number of rotations
// required to move the dial from position `from` to
// position `to` by taking into account that the dial can
// be rotated both clockwise & anti-clockwise
fn dial_cost(n: usize, from: usize, to: usize) -> i32 {
    let a = from.min(to);
    let b = from.max(to);

    (a + n - b).min(b - a) as i32
}

fn build_ring_char_positions(ring: &str) -> HashMap<u8, Vec<usize>> {
    let mut map = HashMap::new();
    for (idx, ch) in ring.bytes().enumerate() {
        map.entry(ch).or_insert_with(|| vec![]).push(idx);
    }
    map
}