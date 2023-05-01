// https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/solutions/3000451/greedy-rust-solution/
impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        if points.len() == 0 {
            return 0;
        }
        
        points.sort_unstable_by(|a, b| a[1].cmp(&b[1]));

        let mut arrows:i32 = 1;
        let mut xStart:i32;
        let mut xEnd:i32; 
        let mut firstEnd:i32 = points[0][1];
        for point in points.into_iter() {
            xStart = point[0];
            xEnd = point[1];
            if firstEnd < xStart {
                arrows+=1;
                firstEnd = xEnd;
            }
        }
        return arrows;
    }
}