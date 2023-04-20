// https://leetcode.com/problems/bus-routes/solutions/751864/idiomatic-rust-code/
use std::collections::{HashMap, HashSet, VecDeque};

pub fn min_buses_to_destination(routes: Vec<Vec<i32>>, start: i32, end: i32) -> Option<usize> {
    if start == end {
        return Some(0);
    }

    let mut adj_busses = HashMap::new();
    let mut start_busses = vec![];
    let mut end_busses = HashSet::new();
    for (bus_one, route_one) in routes.iter().enumerate() {
        let route_one_set: HashSet<_> = route_one.iter().collect();
        if route_one_set.contains(&start) {
            start_busses.push(bus_one);
        }
        if route_one_set.contains(&end) {
            end_busses.insert(bus_one);
        }
        for (bus_two, route_two) in routes.iter().enumerate() {
            if route_two.iter().any(|stop| route_one_set.contains(stop)) {
                adj_busses.entry(bus_one).or_insert(vec![]).push(bus_two);
            }
        }
    }

    let mut cur_busses = start_busses;
    let mut seen_busses = HashSet::new();
    let mut transfers = 1;
    while !cur_busses.is_empty() {
        for bus in cur_busses.iter() {
            if end_busses.contains(bus) {
                return Some(transfers);
            }
            seen_busses.insert(*bus);
        }

        let mut next_busses = vec![];
        for bus in cur_busses.iter() {
            for adj_bus in adj_busses.get(bus).unwrap_or(&Vec::new()) {
                if !seen_busses.contains(adj_bus) {
                    next_busses.push(*adj_bus);
                }
            }
        }

        transfers += 1;
        std::mem::swap(&mut next_busses, &mut cur_busses);
    }

    None
}

pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, s: i32, t: i32) -> i32 {
    match min_buses_to_destination(routes, s, t) {
        Some(x) => x as i32,
        None => -1,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_num_buses() {
        assert_eq!(
            num_buses_to_destination(vec![vec![1, 5, 7], vec![2, 6, 7]], 1, 2),
            2
        );
        assert_eq!(
            num_buses_to_destination(vec![vec![1, 5, 7], vec![2, 6, 7], vec![7, 9, 1]], 1, 9),
            1
        );
    }
}
