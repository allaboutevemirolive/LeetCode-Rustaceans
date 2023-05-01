// https://leetcode.com/problems/minimum-genetic-mutation/solutions/2769849/rust-bfs/
use std::{collections::{HashMap, HashSet, VecDeque}, borrow::Cow};
impl Solution {
    pub fn min_mutation<'a>(start: String, end: String, mut bank: Vec<String>) -> i32 {
        fn is_neighbor(one: &str, two: &str) -> bool {
            if one == two {
                return false;
            }
            
            let mut iter1 = one.chars();
            let mut iter2 = two.chars();
            
            if one.len() < two.len() {
                std::mem::swap(&mut iter1, &mut iter2);
            }
            
            let mut set = false;
            
            for (x, y) in iter1.rev().zip(iter2.rev().chain(std::iter::repeat('a'))) {
                if x != y {
                    if set == true {
                        return false;
                    } else {
                        set = true;
                    }
                }
            }
            
            set
        }
        
        bank.push(start.clone());
        
        let mut graph = HashMap::<Cow<'a, str>, HashSet<Cow<'a, str>>>::new();
        let bank: HashSet<Cow<'a, str>> = bank.into_iter().map(|w| Cow::Owned(w)).collect();
        
        if !bank.contains(&Cow::Owned(end.clone())) {
            return -1;
        }
        
        for x in bank.iter() {
            for y in bank.iter() {
                if is_neighbor(x, y) {
                    (*graph.entry(Cow::Borrowed(x)).or_insert(HashSet::new())).insert(Cow::Borrowed(y));
                }
            }
        }
        
        let start: Cow<'a, str> = Cow::Owned(start);
        let end: Cow<'a, str> = Cow::Owned(end);
        
        let mut visited = HashMap::<Cow<'a, str>, bool>::new();
        for e in bank.iter() {
            visited.insert(Cow::Borrowed(e), false);
        }
                
        let mut q = VecDeque::<(Cow<'a, str>, i32)>::from([(start, 0)]);
        while q.len() > 0 {
            let (cur, i) = q.pop_front().unwrap();
            for n in graph[&cur].iter() {
                if visited[n] == true {
                    continue;
                } else {
                    *(visited.get_mut(n).unwrap()) = true;
                }
                
                if n == &end {
                    return i + 1;
                }
                
                q.push_back((Cow::Borrowed(n), i + 1));
            }
            
        }
        
        -1
    }
}