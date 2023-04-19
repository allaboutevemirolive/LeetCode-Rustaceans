// https://leetcode.com/problems/reconstruct-itinerary/solutions/694484/rust-solution-with-comments/
use std::collections::HashMap;
use std::collections::VecDeque;
use std::iter::FromIterator;

const FROM: usize = 0;
const TO: usize = 1;

pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
    let mut airport_map: HashMap<String, usize> = HashMap::new();
    let mut airport_map_back: HashMap<usize, String> = HashMap::new();

    let mut num_airports = 0;
    
	// Preparation (convert airports name into the array index)
	
    tickets.iter().for_each(|ticket| {
        match airport_map.contains_key(&ticket[FROM]) {
            true => {},
            false => {
                airport_map.insert(ticket[FROM].to_string(), num_airports);
                airport_map_back.insert(num_airports, ticket[FROM].to_string());
                num_airports += 1;
            }
        };

        match airport_map.contains_key(&ticket[TO]) {
            true => {},
            false => {
                airport_map.insert(ticket[TO].to_string(), num_airports);
                airport_map_back.insert(num_airports, ticket[TO].to_string());
                num_airports += 1;
            }
        }
    });

    let mut graph: Vec<Vec<String>> = vec![vec![]; num_airports as usize];
	
	// make adjacency list
    tickets.iter().for_each(|ticket| {
        let from_vertex = airport_map.get(&ticket[FROM]).unwrap();

        let pos = graph[*from_vertex as usize].binary_search(&ticket[TO]).unwrap_or_else(|e| e);
        graph[*from_vertex as usize].insert(pos, ticket[TO].clone());
    });

    let itinerary = dfs(*airport_map.get("JFK").unwrap(), &mut graph, tickets.len(), &airport_map);

    match itinerary {
        Some(it) => {
            return it.iter().map(|airport_number| {airport_map_back.get(airport_number).unwrap().clone()}).collect();
        },
        None => {
            return vec![];
        }
    }

}

// Dfs with backtracking
fn dfs(start: usize, graph: &mut Vec<Vec<String>>, expected_length: usize, airport_map: &HashMap<String, usize>) -> Option<Vec<usize>>  {

    if expected_length == 0 {
		// if we here, it means we used all of the tickets
        return Some(vec![start]);
    }

    for idx in 0..graph[start].len() {
		// get the neighboring vertex
        let edge = &graph[start][idx].clone();
		// remove the vertex from the neighbours list temporarily, so we won't use it twice
        graph[start] = [&graph[start][0..idx], &graph[start][idx+1..]].concat();
        let sub_itinerary = dfs(*airport_map.get(edge).unwrap(), graph, expected_length - 1, airport_map);
		
		// add the vertex back
        graph[start] = [&graph[start][0..idx], &[edge.to_string()], &graph[start][idx..]].concat();

        match sub_itinerary {
            Some(sub_it) => {
                if sub_it.len() == expected_length {
                    return Some([vec![start], sub_it].concat());
                }
            },
            None => {
                continue;
            },
        }
    }

    None
}