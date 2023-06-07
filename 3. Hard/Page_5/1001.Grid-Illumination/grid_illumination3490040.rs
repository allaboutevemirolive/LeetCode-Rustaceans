// https://leetcode.com/problems/grid-illumination/solutions/3490040/rust-numerical-approach/
impl Solution {
    pub fn grid_illumination(n: i32, lamps: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        // Loop over queries and format them nicely
        queries
        .into_iter()
        .map(|query| (query[0], query[1]))
        // Keep track of illumination state
        // Update it after each query
        .scan(
            // Initialize illumination state
            // Loop over lamps and format them nicely
            lamps
            .into_iter()
            .map(|lamp| (lamp[0], lamp[1]))
            // Add all lamps to the list
            .fold(
                Illumination::new(n), 
                |mut illumination, (lamp_x, lamp_y)| {
                    illumination.turn_on_lamp(lamp_x, lamp_y);
                    illumination
            }),
            // Run each query
            |mut illumination, (x, y)| {
                Some(illumination.query(x, y))
            }
        )
        // Collect iterator into output vector
        .collect()
    }
}

struct Illumination {
    n: i32,
    on_lamps: Vec<(i32, i32)>,
}

impl Illumination {
    // Defaults to all lamps off
    pub fn new(n: i32) -> Self {
        Self {
            n,
            on_lamps: Vec::new(),
        }
    }

    pub fn turn_on_lamp(&mut self, x: i32, y: i32) {
        self.on_lamps.push((x, y));
    }

    pub fn is_illuminated(&self, x: i32, y: i32) -> bool {
        if x.max(y) >= self.n {
            return false;
        }

        self.on_lamps
        .iter()
        .any(|&(lamp_x, lamp_y)| {
            let horizontal = (lamp_x == x);
            let vertical = (lamp_y == y);
            let down_right = ((lamp_x - lamp_y) == (x - y));
            let down_left = ((lamp_x + lamp_y) == (x + y));
            horizontal || vertical || down_right || down_left
        })
    }

    // Remove any lamps that are within 1 on both axes
    pub fn turn_off_box(&mut self, x: i32, y: i32) {
        self.on_lamps.retain(|&(lamp_x, lamp_y)| (lamp_x - x).abs() > 1 || (lamp_y - y).abs() > 1);
    }

    // Get if the lamp is illuminated, then turn off boxes
    pub fn query(&mut self, x: i32, y: i32) -> i32 {
        let output = if self.is_illuminated(x, y) {
            1
        } else {
            0
        };

        self.turn_off_box(x, y);
        output
    }
}