// https://leetcode.com/problems/minimum-number-of-taps-to-open-to-water-a-garden/solutions/882701/accepted-rust-o-nlogn-solution-using-treemap/
        let ranges = Vec::from_iter((0..=n).zip(ranges).map(|(i, r)| (i - r, i + r)));
        let mut indices = Vec::from_iter(0..=n as usize);
        indices.sort_by(|&i, &j| ranges[i].0.cmp(&ranges[j].0));

        let mut dp = BTreeMap::new();
        dp.insert(0, 0);
        for ind in indices {
            if ranges[ind].1 <= *dp.iter().next_back().unwrap().0 {
                continue;
            }
            match dp
                .range((Bound::Included(ranges[ind].0), Bound::Unbounded))
                .next()
            {
                Some((&_, &taps)) => {
                    dp.insert(ranges[ind].1, taps + 1);
                    if ranges[ind].1 >= n {
                        break;
                    }
                }
                None => break,
            }
        }
        let (&d, &taps) = dp.iter().next_back().unwrap();
        if d >= n {
            taps
        } else {
            -1
        }