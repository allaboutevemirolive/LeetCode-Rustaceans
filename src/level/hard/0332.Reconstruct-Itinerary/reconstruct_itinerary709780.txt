// https://leetcode.com/problems/reconstruct-itinerary/solutions/709780/rust-4ms/
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

// https://leetcode.com/problems/reconstruct-itinerary/discuss/78768/Short-Ruby-Python-Java-C%2B%2B
#[derive(Debug, Default, Clone, Eq, PartialEq)]
struct Airport {
    name: String,
}

impl std::cmp::PartialOrd for Airport {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.name.partial_cmp(&self.name)
    }
}

impl std::cmp::Ord for Airport {
    fn cmp(&self, other: &Self) -> Ordering {
        other.name.cmp(&self.name)
    }
}

pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
    fn visit(
        airport: String,
        targets: &mut HashMap<String, BinaryHeap<Airport>>,
        route: &mut Vec<String>,
    ) {
        // println!("targets = {:?}", targets);
        // println!("route = {:?}", route);
        while let Some(x) = targets.get_mut(&airport) {
            if !x.is_empty() {
                visit(x.pop().unwrap().name, targets, route)
            } else {
                break;
            }
        }
        route.insert(0, airport)
    }

    let mut ans = vec![];
    let mut targets = HashMap::<String, BinaryHeap<Airport>>::new();

    for ticket in tickets {
        targets
            .entry(ticket[0].clone())
            .or_insert(BinaryHeap::new())
            .push(Airport {
                name: ticket[1].clone(),
            });
    }
    //    println!("targets = {:?}", targets);
    visit("JFK".to_string(), &mut targets, &mut ans);

    ans
}
