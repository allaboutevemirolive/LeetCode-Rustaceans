// https://leetcode.com/problems/bus-routes/solutions/873659/rust-translated-44ms-100/
impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, s: i32, t: i32) -> i32 {
        use std::collections::{HashMap, HashSet, VecDeque};

        let n = routes.len();
        let mut stop2routes = HashMap::<i32, HashSet<i32>>::new();
        for (i, route) in routes.iter().enumerate() {
            for &j in route {
                stop2routes.entry(j).or_default().insert(i as i32);
            }
        }
        let mut q = VecDeque::<(i32, i32)>::new();
        let mut visited = HashSet::<i32>::new();
        q.push_back((s, 0));
        visited.insert(s);
        let mut visited_routes = vec![false; n];
        while !q.is_empty() {
            let (stop, bus) = q.pop_front().unwrap();
            if stop == t {
                return bus;
            }
            for &i in stop2routes.get(&stop).unwrap() {
                if visited_routes[i as usize] {
                    continue;
                }
                for &j in &routes[i as usize] {
                    if !visited.contains(&j) {
                        visited.insert(j);
                        q.push_back((j, bus + 1))
                    }
                }
                visited_routes[i as usize] = true;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_buses_to_destination() {
        assert_eq!(
            Solution::num_buses_to_destination(vec![vec![1, 2, 7], vec![3, 6, 7]], 1, 3),
            2
        );
    }
}