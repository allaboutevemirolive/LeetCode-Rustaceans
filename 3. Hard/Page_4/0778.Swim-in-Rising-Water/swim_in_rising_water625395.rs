// https://leetcode.com/problems/swim-in-rising-water/solutions/625395/rust-0ms-straightforward-dijkstra-soln/
use std::cmp::Ordering;
use std::collections::BinaryHeap;

// Struct for an x,y location and its height
// (sorted in priority queue where front is the min)
#[derive(PartialEq, Debug)]
struct LocAndHei(i32, i32, i32);

impl Eq for LocAndHei {}

impl PartialOrd for LocAndHei {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.2.partial_cmp(&self.2)
    }
}

impl Ord for LocAndHei {
    fn cmp(&self, other: &LocAndHei) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

// Seems to be required for there to be an impl.
//struct Solution{}

impl Solution {

    // To solve this problem we will use a dijkstra priority queue.
    // At each step pull the lowest unprocessed item
    pub fn swim_in_water(mut grid: Vec<Vec<i32>>) -> i32 {

        let mut minheap = BinaryHeap::new();
        let mut water_height = grid[0][0];
        let grid_dimension = grid.len() - 1;

        // Init the heap
        minheap.push(LocAndHei(0,0,water_height));
        grid[0][0] = -1;

        // dijkstra search
        loop {
            if let Some(LocAndHei(x,y,height)) = minheap.pop() {

                let item = LocAndHei(x,y,height);
                let x_casted = x as usize;
                let y_casted = y as usize;

                // Update the water height
                if item.2 > water_height
                {
                    water_height = item.2;
                }

                //println!("considering: {:?}, water height so far: {:?}", item, water_height);

                // The item popped is the bottom right corner, we are done
                if item.0 == grid_dimension as i32 && item.0 == item.1 {
                    //println!("finishing. Grid dimension: {:?} {:?}", grid_dimension, item);
                    break;
                }

                // Otherwise, push on new paths if they are not marked with a -1 already
                // up
                if x_casted > 0 && grid[x_casted-1][y_casted] != -1 {
                    //println!("pushing up");
                    minheap.push(LocAndHei(x-1, y, grid[x_casted-1][y_casted]));
                    grid[x_casted-1][y_casted] = -1;
                }

                // Down
                if x_casted+1 <= grid_dimension && grid[x_casted+1][y_casted] != -1 {
                    //println!("pushing down");
                    minheap.push(LocAndHei(x+1, y, grid[x_casted+1][y_casted]));
                    grid[x_casted+1][y_casted] = -1;
                }

                // left
                if y_casted > 0 && grid[x_casted][y_casted-1] != -1 {
                    //println!("pushing left");
                    minheap.push(LocAndHei(x, y-1, grid[x_casted][y_casted-1]));
                    grid[x_casted][y_casted-1] = -1;
                }

                // right
                if y_casted+1 <= grid_dimension && grid[x_casted][y_casted+1] != -1 {
                    //println!("pushing right");
                    minheap.push(LocAndHei(x, y+1, grid[x_casted][y_casted+1]));
                    grid[x_casted][y_casted+1] = -1;
                }

                //println!("{:?}\n\n", grid);
            } else {
                //println!("PANIC");
                panic!("Failed to get min item from heap")
            }
        }

        return water_height;
    }
}