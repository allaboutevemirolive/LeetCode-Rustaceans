// https://leetcode.com/problems/dinner-plate-stacks/solutions/930743/2-rust-solutions-faster-than-100-using-binaryheap/
use std::collections::BinaryHeap;
use std::convert::{TryInto, TryFrom};
use std::cmp::Reverse;

#[derive(Debug)]
struct DinnerPlates {
    stacks: Vec<Vec<i32>>,
    not_full_queue: BinaryHeap<Reverse<usize>>,
    max_capacity: usize,
    last_poppable: usize,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl DinnerPlates {

    fn new(capacity: i32) -> Self {
        Self{
            stacks: Vec::new(),
            not_full_queue: BinaryHeap::new(),
            max_capacity: capacity.try_into().unwrap(),
            last_poppable: 0
        }
    }
    
    fn push(&mut self, val: i32) {
        if let Some(&Reverse(left_most_non_empty)) = self.not_full_queue.peek(){
            self.stacks[left_most_non_empty].push(val);
            if self.stacks[left_most_non_empty].len() >= self.max_capacity{
                self.not_full_queue.pop();
            }
            if self.last_poppable < left_most_non_empty{
                self.last_poppable = left_most_non_empty;
            }
        }
        else{
            self.stacks.push(Vec::with_capacity(self.max_capacity));
            self.stacks.last_mut().unwrap().push(val);
            self.last_poppable = self.stacks.len() - 1;
            if self.max_capacity > 1 {
                self.not_full_queue.push(Reverse(self.stacks.len()-1));
            }
        }
    }
    
    fn pop(&mut self) -> i32 {
        self.pop_at_idx(self.last_poppable)
    }
    
    fn pop_at_stack(&mut self, index: i32) -> i32 {
        self.pop_at_idx(index.try_into().unwrap())
    }
    
    fn pop_at_idx(&mut self, index: usize)->i32{
        if index >= self.stacks.len(){
            return -1;
        }
        let was_full = self.stacks[index].len() == self.max_capacity;
        let result = self.stacks[index].pop().unwrap_or(-1);
        if was_full {
            self.not_full_queue.push(Reverse(index));
        }
        if index == self.last_poppable && self.stacks[index].is_empty(){
            while self.stacks[self.last_poppable].is_empty() && self.last_poppable > 0{
                self.last_poppable -= 1;
            }
        }
        result
    }
}