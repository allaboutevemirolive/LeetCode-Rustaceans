// https://leetcode.com/problems/falling-squares/solutions/845821/rust-translated-8ms-100/
impl Solution {
    pub fn falling_squares(mut positions: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::BTreeMap;

        let mut map = BTreeMap::<i32, i32>::new();
        map.insert(0, 0);
        let mut max = 0;
        let mut ans = vec![];
        for pos in &positions {
            let start = pos[0];
            let end = pos[0] + pos[1];
            let mut h = 0;
            let mut k = 0;
            if let Some((&key, &height)) = map.range(..=start).last() {
                h = height;
                k = key;
            }
            let mut iter = map.range(k..).skip(1);
            while let Some((&key, &height)) = iter.next() {
                if key >= end {
                    break;
                }
                h = std::cmp::max(h, height);
            }
            h += pos[1];
            max = std::cmp::max(max, h);
            ans.push(max);

            if let Some((&tail, &height)) = map.range(..end).last() {
                map.insert(start, h);
                map.insert(end, height);
            }
            let mut iter = map.range(start..).skip(1);
            let mut list = vec![];
            while let Some((&key, &height)) = iter.next() {
                if key < end {
                    list.push(key)
                } else {
                    break;
                }
            }
            for key in list {
                map.remove(&key);
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_falling_squares() {
        assert_eq!(
            Solution::falling_squares(vec![vec![1, 2], vec![2, 3], vec![6, 1]]),
            vec![2, 5, 5]
        );
    }

    #[test]
    fn test_falling_squares_02() {
        assert_eq!(
            Solution::falling_squares(vec![vec![100, 100], vec![200, 100]]),
            vec![100, 100]
        );
    }
}