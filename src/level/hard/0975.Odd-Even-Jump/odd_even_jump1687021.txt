// https://leetcode.com/problems/odd-even-jump/solutions/1687021/rust-solution-use-a-sorted-vec/
impl Solution {
    pub fn odd_even_jumps(arr: Vec<i32>) -> i32 {
        let len = arr.len();
        assert!(len > 0);

        // visied jump states
        let mut odds = vec![false; len];
        let mut evens = vec![false; len];

        // the end is always a good jump
        odds[len - 1] = true;
        evens[len - 1] = true;
        let mut good = 1;

        let mut sorted_arr: Vec<(i32, usize)> = Vec::with_capacity(len);
        sorted_arr.push((arr[len - 1], len - 1));

        for i in (0..len - 1).rev() {
            let item = (arr[i], i);
            let idx = sorted_arr.binary_search(&item).unwrap_or_else(|x| x);
            sorted_arr.insert(idx, item);

            // 1. check odd jump
            let mut next_step: Option<usize> = None;
            if idx < sorted_arr.len() - 1 {
                next_step = Some(idx + 1);
            }

            if let Some(next_step) = next_step {
                let next = sorted_arr[next_step];
                let even = evens[next.1];
                odds[i] = even;
                if even {
                    good += 1;
                }
            }

            // 2. check even jump
            next_step = None;
            // 2.1. if current .. next, next == current
            if idx < sorted_arr.len() - 1 && sorted_arr[idx + 1].0 == item.0 {
                next_step = Some(idx + 1);
            } else {
                // 2.2 if ... prev prev current
                for i in (0..idx).rev() {
                    // get minimum index
                    if i > 0 && sorted_arr[i - 1].0 == sorted_arr[i].0 {
                        continue;
                    }

                    next_step = Some(i);
                    break;
                }
            }

            if let Some(next_step) = next_step {
                let next = sorted_arr[next_step];
                evens[i] = odds[next.1];
            }
        }

        good
    }
}