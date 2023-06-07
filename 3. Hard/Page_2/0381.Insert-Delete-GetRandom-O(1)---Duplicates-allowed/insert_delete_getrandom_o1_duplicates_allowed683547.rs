// https://leetcode.com/problems/insert-delete-getrandom-o1-duplicates-allowed/solutions/683547/rust-code/
use rand::prelude::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct RandomizedCollection {
    rng: ThreadRng,
    numbers: Vec<i32>,
    map: HashMap<i32, HashSet<i32>>,
}

impl RandomizedCollection {
    /** Initialize your data structure here. */
    fn new() -> Self {
        RandomizedCollection {
            rng: Default::default(),
            numbers: vec![],
            map: Default::default(),
        }
    }

    /** Inserts a value to the collection. Returns true if the collection did not already contain the specified element. */
    fn insert(&mut self, val: i32) -> bool {
        let contained = self.map.contains_key(&val);
        if !contained {
            self.map.insert(val, Default::default());
        }
        self.map.get_mut(&val).unwrap().insert(self.numbers.len() as i32);
        self.numbers.push(val);

        !contained
    }

    /** Removes a value from the collection. Returns true if the collection contained the specified element. */
    fn remove(&mut self, val: i32) -> bool {
        return match self.map.get_mut(&val) {
            None => { false }
            Some(x) => {
                let i = x.iter().next();
                match i {
                    None => { return false; }
                    Some(&z) => {
                        x.remove(&z);
                        let len = self.numbers.len();
                        if z < (len - 1) as i32 {
                            let last_one = self.numbers[len - 1];
                            self.numbers[z as usize] = last_one;
                            match self.map.get_mut(&last_one) {
                                Some(s) => {
                                    s.remove(&((len - 1) as i32));
                                    s.insert(z);
                                },
                                None => { return false; }
                            }
                        }
                    }
                }
                self.numbers.pop();
                if let Some(x) = self.map.get(&val) {
                    if x.is_empty() { self.map.remove(&val); }
                };
                true
            }
        };
    }

    /** Get a random element from the collection. */
    fn get_random(&mut self) -> i32 {
        self.numbers[self.rng.gen_range(0, self.numbers.len())]
    }
}

