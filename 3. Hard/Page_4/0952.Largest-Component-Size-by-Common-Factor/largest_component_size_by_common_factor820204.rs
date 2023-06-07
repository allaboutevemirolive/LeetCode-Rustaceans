// https://leetcode.com/problems/largest-component-size-by-common-factor/solutions/820204/rust-solution-40ms-runtime/
#[derive(Clone, Copy)]
enum CountEntry {
    Count(i32),
    Forward(i32),
}

pub fn prime_factors(mut n: i32) -> impl Iterator<Item = i32> {
    const PRIMES: [i32; 65] = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67,
        71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149,
        151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229,
        233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313
    ];

    let mut i = 0;

    std::iter::from_fn(move || {
        if n == 1 { return None; }
        loop {
            let p = PRIMES.get(i).copied().unwrap_or(n);
            if n % p == 0 {
                while n % p == 0 { n /= p; }
                return Some(p);
            }
            i += 1;
        }
    })
}

impl Solution {
    pub fn largest_component_size(a: Vec<i32>) -> i32 {
        let mut counts = std::collections::HashMap::new();
        
        for n in a {
            let mut prime_factors = prime_factors(n);
            
            let mut entry = prime_factors.next().unwrap_or(1);
            let sum_entry = loop {
                match counts.entry(entry).or_insert(CountEntry::Count(0)) {
                    CountEntry::Count(count) => { *count += 1; break entry },
                    CountEntry::Forward(forward) => { entry = *forward; }
                }
            };
            
            'factors: for factor in prime_factors {
                let mut entry = factor;
                loop {
                    if entry == sum_entry { continue 'factors; }
                    match *counts.entry(entry).or_insert(CountEntry::Forward(sum_entry)) {
                        CountEntry::Count(to_sum) => {
                            match counts.get_mut(&sum_entry).unwrap() {
                                CountEntry::Count(sum_count) => *sum_count += to_sum,
                                CountEntry::Forward(_) => unreachable!(),
                            }
                            counts.insert(entry, CountEntry::Forward(sum_entry));
                        }
                        CountEntry::Forward(forward) => {
                            counts.insert(entry, CountEntry::Forward(sum_entry));
                            entry = forward;
                        }
                    }
                }
            }
        }
        
        counts
            .into_iter()
            .filter_map(|(_, count_entry)| match count_entry {
                CountEntry::Count(count) => Some(count),
                CountEntry::Forward(_) => None,
            })
            .max()
            .unwrap()
    }
}