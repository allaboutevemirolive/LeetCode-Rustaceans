// https://leetcode.com/problems/reconstruct-itinerary/solutions/3558525/rust-performant-solution-3ms-2-2mb-hierholzer-s-algorithm-dfs/
impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        use std::collections::{HashMap, VecDeque};
        const START: &'static str = "JFK";
        let mut route = VecDeque::new();
        let mut stack = vec![START.to_string()];
        let mut routes = HashMap::<String, Vec<String>>::new();

        for t in tickets {
            routes
                .entry(t[0].to_string())
                .and_modify(|dests| (*dests).push(t[1].to_string()))
                .or_insert(vec![t[1].to_string()]);
        }

        routes.values_mut().for_each(
            |dests| dests.sort_by(|a, b| b.cmp(&a)));

        while let Some(current) = stack.pop() {
            if let Some(dest_list) = routes.get_mut(&current) {
                if let Some(next) = dest_list.pop() {
                    stack.push(current.clone());
                    stack.push(next);
                } else {
                    route.push_front(current);
                }
            } else {
                route.push_front(current);
            }
        }

        route.into_iter().collect()
    }
}