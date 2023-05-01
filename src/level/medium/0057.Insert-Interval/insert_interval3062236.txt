// https://leetcode.com/problems/insert-interval/solutions/3062236/rust-solution/
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![new_interval];
        }

        let mut res = vec![];
        let mut cur_vec = Vec::with_capacity(2);
        let (mut start, mut end) = (new_interval.first(), new_interval.last());

        for mut interval in intervals {
            match (start, end) {
                (None, None) => cur_vec = interval,
                (Some(&s), Some(_)) if s > interval[1] => cur_vec = interval,
                (Some(&s), Some(&e)) if e <= interval[1] => {
                    if e < interval[0] {
                        res.push(new_interval.clone());
                    } else {
                        interval[0] = s.min(interval[0]);
                    }
                    start = None;
                    end = None;
                    cur_vec = interval;
                }

                (Some(&s), Some(_)) if s <= interval[1] => {
                    cur_vec.push(s.min(interval[0]));
                    start = None;
                }

                (None, Some(&e)) if e <= interval[1] => {
                    if e < interval[0] {
                        cur_vec.push(e);
                        res.push(cur_vec.clone());
                        cur_vec = interval;
                    } else {
                        cur_vec.push(interval[1]);
                    }
                    end = None;
                }
                _ => (),
            }

            if cur_vec.len() == 2 {
                res.push(cur_vec.clone());
                cur_vec.clear();
            }
        }

        match (start, end) {
            (Some(_), _) => res.push(new_interval),
            (None, Some(&e)) => {
                cur_vec.push(e);
                res.push(cur_vec.clone());
            }
            _ => (),
        }
        res
    }
}