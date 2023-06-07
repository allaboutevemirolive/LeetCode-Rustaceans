// https://leetcode.com/problems/dinner-plate-stacks/solutions/2801573/rust-using-a-btreeset/
use std::collections::BTreeSet;

const NO_VALUE: i32 = -1;

struct DinnerPlates {
    // how many elements each stack can have 
    capacity: usize,

    // the stacks themselves
    stacks: Vec<Vec<i32>>,

    // track the indexes of the non-full stacks
    non_full: BTreeSet<usize>,
}

impl DinnerPlates {
    fn new(capacity: i32) -> Self {
        assert!(capacity > 0);
        Self {
            capacity: capacity as usize,
            stacks: vec![],
            non_full: BTreeSet::new(),
        }
    }

    fn push(&mut self, val: i32) {
        let idx = match self.non_full.iter().next().copied() {
            None => {
                /// If there are no non-full stacks, or even no stacks at all, 
                /// then we have to allocate a new stack
                self.stacks.push(vec![]);
                let last_stack_idx = self.stacks.len() - 1;

                // If the allowed capacity is more than 1 element, we have to 
                // register it it with the non_full tracker.
                //
                // If the allowed capacity is only 1 element, then we can skip 
                // adding it to the "non_full" set because we would have to 
                // remove it immediately
                if self.capacity > 1 {
                    self.non_full.insert(last_stack_idx);
                }

                last_stack_idx
            }

            Some(idx) => {
                // If there is a non-full stack, check if it would become full 
                // after we add a new element to it. If that;s the case, then 
                // remove it from the "non_full" set
                if self.capacity == self.stacks[idx].len() + 1 {
                    self.non_full.remove(&idx);
                }

                idx
            }
        };

        // Add the new element to the selected stack
        self.stacks[idx].push(val);
    }

    fn pop(&mut self) -> i32 {
        // There are no non-empty stacks, thus we have to return the "no answer" value
        if self.stacks.is_empty() {
            return NO_VALUE;
        }

        // Get the value from the rightmost non-empty stack
        let rightmost_stack = self.stacks.len() - 1;
        let value = self.stacks[rightmost_stack].pop().unwrap();

        // Check if the rightmost stack was full. In that case, register it with the non-full tracker
        // If it was full, but the capacity was 1, then do not register it, because we will
        // remove it either way
        if self.capacity > 1 && self.stacks[rightmost_stack].len() + 1 == self.capacity {
            self.non_full.insert(rightmost_stack);
        }

        // Remove the empty stacks at the right until we reach a non-empty stack
        while !self.stacks.is_empty() && self.stacks[self.stacks.len() - 1].is_empty() {
            self.non_full.remove(&(self.stacks.len() - 1));
            self.stacks.pop();
        }

        value
    }

    fn pop_at_stack(&mut self, index: i32) -> i32 {
        assert!(index >= 0);
        let index = index as usize;

        // There is no stack at the provided index
        if index >= self.stacks.len() {
            return NO_VALUE;
        }

        // Special case - removing from the rightmost stack
        // Call the other method to handle the removal of all
        // adjacent empty stacks to the left
        if index == self.stacks.len() - 1 {
            return self.pop();
        }

        // The selected stack was full, so register it with 
        // the non-full stack tracker
        if self.stacks[index].len() == self.capacity {
            self.non_full.insert(index);
        };

        self.stacks[index].pop().unwrap_or(NO_VALUE)
    }
}