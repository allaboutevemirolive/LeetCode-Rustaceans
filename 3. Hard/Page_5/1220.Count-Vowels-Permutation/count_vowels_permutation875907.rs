// https://leetcode.com/problems/count-vowels-permutation/solutions/875907/concise-rust-solution/
        let from = vec![vec![1, 2, 4], vec![0, 2], vec![1, 3], vec![2], vec![2, 3]];
        let rem = 10i32.pow(9) + 7;
        let mut cur = vec![1; 5];
        for _ in 1..n {
            let temp = cur;
            cur = Vec::from_iter(
                from.iter()
                    .map(|v| v.into_iter().fold(0, |acc, &i| (temp[i] + acc) % rem)),
            );
        }
        cur.into_iter().fold(0, |acc, n| (acc + n) % rem)
