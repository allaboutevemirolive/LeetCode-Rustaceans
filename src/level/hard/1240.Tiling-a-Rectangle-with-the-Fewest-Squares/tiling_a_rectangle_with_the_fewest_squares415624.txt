// https://leetcode.com/problems/tiling-a-rectangle-with-the-fewest-squares/solutions/415624/rust-solution/
impl Solution {
    pub fn tiling_rectangle(n: i32, m: i32) -> i32 {
        struct Skyline {
            heights: Vec<i32>,
            components: Vec<i32>,
            target_height: i32,
            prev_steps: Vec<(i32, i32)>
        };

        impl Skyline {
            fn new(target_height: i32, heights: Vec<i32>, components: Vec<i32>) -> Skyline {
                Skyline {
                    heights,
                    components,
                    target_height,
                    prev_steps: vec![]
                }
            }
            fn component_length(&self) -> i32 {
                self.components.len() as i32
            }
            fn is_complete(&self) -> bool {
                self.heights.iter().all(|&h| h == self.target_height)
            }

            fn add(&mut self, block: i32) {
                let bottom_left_index = self.find_bottom_left();
                self.components.push(block);
                self.prev_steps.push((bottom_left_index, block));
                for i in bottom_left_index..bottom_left_index + block {
                    self.heights[i as usize] += block;
                }
            }

            fn revert(&mut self) {
                if let Some((index, block)) = self.prev_steps.pop() {
                    self.components.pop();
                    for i in index..index + block {
                        self.heights[i as usize] -= block;
                    }
                }
            }

            fn can_fit(&self, block: i32) -> bool {
                let bottom_left_index = self.find_bottom_left();
                let mut result = true;
                
                if bottom_left_index + block > self.heights.len() as i32 {
                    return false;
                }
                for i in bottom_left_index..bottom_left_index + block {
                    if self.heights[i as usize] + block > self.target_height {
                        result = false;
                    }
                }
                result
            }

            fn find_bottom_left(&self) -> i32 {
                let mut result = 0;
                let mut min_height = self.heights[0];
                for (i, &h) in self.heights.iter().enumerate() {
                    if h < min_height {
                        min_height = h;
                        result = i as i32;
                    }
                }
                result
            }
        }

        fn helper(skyline: &mut Skyline, best_result: i32) -> i32 {
            let mut result = best_result;
            let building_blocks: Vec<i32> = (1..=13).rev().collect();
            let components_length = skyline.component_length();
            if components_length >= best_result {
                return best_result;
            }
            if skyline.is_complete() {
                return components_length;
            }
            for block in building_blocks {
                if skyline.can_fit(block) {
                    skyline.add(block);
                    let temp_result = helper(skyline, result);
                    result = std::cmp::min(result, temp_result);
                    skyline.revert();
                }
            }
            result
        };

        let mut heights = vec![];
        for _ in 0..n {
            heights.push(0);
        }
        let mut base_skyline = Skyline::new(m, heights, vec![]);
        helper(&mut base_skyline, std::i32::MAX)
    }
}