// https://leetcode.com/problems/number-of-atoms/solutions/224060/rust-iterative-solution-0ms/
use std::collections::HashMap;

struct CounterMap {
    counter : HashMap<usize, i32>,    
}

impl CounterMap {
    pub fn new() -> CounterMap {
        CounterMap {
            counter : HashMap::new(),
        }
    }

    // increment the count by 1
    pub fn inc(&mut self, index : usize) {
        let entry = self.counter.entry(index).or_insert(0);
        (*entry) += 1;
    }
    
    // multiply all entries by x
    pub fn multiply(&mut self, x : i32) {
        self.counter = self.counter.iter().map(|(key,value)| (*key, value * x)).collect();
    }
    
    // merge a pair of CounterMap's together
    pub fn merge(&mut self, other : &CounterMap) {
        for (k, v) in &other.counter {
            let entry = self.counter.entry(*k).or_insert(0);
            (*entry) += v;
        }
    }
}

impl Solution {
    pub fn count_of_atoms(formula: String) -> String {
        let mut char_stack = Vec::new();
        let mut digit_stack = Vec::new();
        let mut counter_stack = Vec::new();
        
        let mut string_holder = HashMap::new();
        let mut index_holder = HashMap::new();
        let mut string_holder_index = 0;
        
        let mut old = CounterMap::new();
        let mut prev = CounterMap::new();
        
        for c in (formula + "+").chars() {
            let mut current = CounterMap::new();
            let mut merge = false;
            
            if ((!c.is_alphabetic() || c.is_uppercase()) && char_stack.len() > 0) {
                // convert the char stack to a string
                let atom : String = char_stack.into_iter().collect::<String>();
                let mut index = 0;
                
                // if we've already seen this atom before, get its index
                if (!string_holder.contains_key(&atom)) {
                    string_holder.insert(atom.clone(), string_holder_index);
                    index_holder.insert(string_holder_index, atom.clone());
                    index = string_holder_index;
                    string_holder_index += 1;
                } else {
                    index = string_holder[&atom];
                }
                
                // increment this letter by 1
                current.inc(index);
                
                // clear the char_stack
                char_stack = Vec::new();
                
                // we will perform a merge later
                merge = true;
            }
            
            if (!c.is_digit(10) && digit_stack.len() > 0) {
                // convert the digit stack to a number
                let num = digit_stack.into_iter()
                                     .collect::<String>()
                                     .parse::<i32>()
                                     .unwrap();
                
                // now multiply previous by num
                prev.multiply(num);
                
                // clear out the digit stack
                digit_stack = Vec::new();
                
                // mark our intent to merge
                merge = true;
            }
            
            // this is somewhat lazy, but we added a + to the end of formula
            // so we don't have to repeat the above code after the for loop
            // to deal with the case where the digit / character stack is non-empty
            if (c != '+') {
                if (c.is_alphabetic()) {
                    char_stack.push(c);
                } else if (c.is_digit(10)) {
                    digit_stack.push(c);
                } else if (c == '(') {
                    // merge the current state, then push that onto counter_stack
                    old.merge(&prev);
                    old.merge(&current);
                    counter_stack.push(old);
                    
                    old = CounterMap::new();
                    prev = CounterMap::new();
                    
                    // do not perform a global merge here
                    merge = false;
                } else if (c == ')') {
                    // merge the current state, then set that as previous
                    old.merge(&prev);
                    old.merge(&current);
                    prev = old;
                    
                    current = CounterMap::new();
                    
                    // our old state is what our state was when we opened our parentheses
                    old = counter_stack.pop().unwrap();
                    
                    // do not perform a global merge here
                    merge = false;
                }
            }
            
            if (merge) {
                // merge old and prev together
                old.merge(&prev);
                prev = current;
            }
        }
        
        // merge prev and old at the very end
        // current should always be empty here
        old.merge(&prev);
        
        // sort the atoms in alphabetical order
        let mut atoms = string_holder.keys().collect::<Vec<_>>();
        atoms.sort();
        
        // create the output string
        let mut output : String = String::new();
        
        for atom in atoms {
            let num = old.counter.get(string_holder.get(atom)
                                                   .unwrap())
                                 .unwrap();
            
            output += atom;
            
            // we only display the count if the count is > 1
            if (*num > 1) {
                output += &num.to_string();
            }
        }
        
        output
    }
}