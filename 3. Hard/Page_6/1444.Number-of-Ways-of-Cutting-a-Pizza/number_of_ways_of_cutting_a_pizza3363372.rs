// https://leetcode.com/problems/number-of-ways-of-cutting-a-pizza/solutions/3363372/rust-top-down-dp/
impl Solution {
    pub fn ways(pizza: Vec<String>, k: i32) -> i32 {
        ways(pizza, k)
    }
}

const APPLE: u8 = b'A';
const MOD: i32 = 10i32.pow(9) + 7;

pub fn ways(pizza: impl AsRef<[String]>, k: i32) -> i32 {
    let pizza = pizza
        .as_ref()
        .into_iter()
        .map(|x| x.as_bytes())
        .collect::<Vec<_>>();

    // Fast path: there is no pizza
    if pizza.len() == 0 || pizza[0].len() == 0 || k <= 0 {
        return 0;
    }

    let cuts = (k - 1) as usize;
    let rows = pizza.len();
    let cols = pizza[0].len();

    let mut cache = vec![vec![vec![-1; cuts + 1]; rows + 1]; cols + 1];
    dfs(&mut cache, &pizza, 0, pizza[0].len(), 0, pizza.len(), cuts)
}

fn dfs(
    cache: &mut Vec<Vec<Vec<i32>>>,
    pizza: &[&[u8]],
    l: usize,
    r: usize,
    t: usize,
    b: usize,
    cuts: usize,
) -> i32 {
    if cuts == 0 {
        return has_apple(pizza, l, r, t, b) as i32;
    }

    if cache[l][t][cuts] != -1 {
        return cache[l][t][cuts];
    }

    let mut ways = 0;

    // Cut horizontally
    let mut prev_had_apple = false;
    for row in t..b - 1 {
        if prev_had_apple || has_apple(pizza, l, r, t, row + 1) {
            // if a previous row had an apple, all following rows
            // will have an apple too, because the piece, that will
            // be cut will still contain that apple, so no need to check again
            prev_had_apple = true;

            let new_ways = dfs(cache, pizza, l, r, row + 1, b, cuts - 1);
            if new_ways == 0 {
                break;
            }

            ways = (ways + new_ways) % MOD;
        }
    }

    // Cut vertically
    prev_had_apple = false;
    for col in l..r - 1 {
        if prev_had_apple || has_apple(pizza, l, col + 1, t, b) {
            // if a previous row had an apple, all following rows
            // will have an apple too, because the piece, that will
            // be cut will still contain that apple, so no need to check again
            prev_had_apple = true;

            let new_ways = dfs(cache, pizza, col + 1, r, t, b, cuts - 1);
            if new_ways == 0 {
                break;
            }

            ways = (ways + new_ways) % MOD;
        }
    }

    cache[l][t][cuts] = ways;
    ways
}

fn has_apple(pizza: &[&[u8]], l: usize, r: usize, t: usize, b: usize) -> bool {
    pizza[t..b]
        .iter()
        .map(|x| x[l..r].iter())
        .flatten()
        .any(|&x| APPLE == x)
}