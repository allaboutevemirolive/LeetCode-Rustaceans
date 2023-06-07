// https://leetcode.com/problems/construct-target-array-with-multiple-sums/solutions/512087/rust-readable-and-fast-solution-using-binaryheap/
        use std::collections::BinaryHeap;

        let mut pq: BinaryHeap<i32> = BinaryHeap::new();
        let mut sum: i64 = 0;
        let limit: i64 = std::i32::MAX as i64 * 2;
        for i in 0..target.len() {
            pq.push(target[i]);
            sum += target[i] as i64;
            if sum > limit {
                return false;
            }
        }
        while !pq.is_empty() {
            let x = pq.pop().unwrap();
            if x == 1 {
                return true;
            }
            let y: i32 = sum as i32 - x;
            if y >= x {
                return false;
            }
            pq.push(x - y);
            sum = x as i64;
        }
        return true;