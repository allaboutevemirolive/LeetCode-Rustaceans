// https://leetcode.com/problems/perfect-rectangle/solutions/867159/rust-16ms-2mb-time-o-n-space-o-n/

use std::cmp::{max, min};
use std::collections::HashSet;
use std::i32::{MAX, MIN};
impl Solution {
    // Time O(N) Space O(N)
    // reference https://www.youtube.com/watch?v=8JM_dyOu_JY
    pub fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
      let mut corners = HashSet::new();
      let mut area = 0;

      for rect in rectangles.iter() {
        let p1 = (rect[0], rect[1]);
        let p2 = (rect[2], rect[1]);
        let p3 = (rect[2], rect[3]);
        let p4 = (rect[0], rect[3]);

        for p in vec![p1, p2, p3, p4] {
          if !corners.insert(p) {
            corners.remove(&p);
          }
        }

        area += (p3.0 - p1.0) * (p3.1 - p1.1);
      }

      if corners.len() != 4 {
        return false;
      }

      let (mut x1, mut y1, mut x2, mut y2) = (MAX, MAX, MIN, MIN);
      for p in corners.iter(){
        x1 = min(p.0, x1);
        y1 = min(p.1, y1);
        
        x2 = max(p.0, x2);
        y2 = max(p.1, y2);
      }

      area == (x2 - x1 ) * (y2 - y1)
    }
}

