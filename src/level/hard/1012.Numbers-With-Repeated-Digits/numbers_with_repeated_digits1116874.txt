// https://leetcode.com/problems/numbers-with-repeated-digits/solutions/1116874/beautiful-and-elegant-rust-100-100/
impl Solution {
    pub fn num_dup_digits_at_most_n(n: i32) -> i32 {
        match V.binary_search(&n) {
            Ok(idx) => n - idx as i32,
            Err(idx) => n + 1 - idx as i32,
        }
    }
}

lazy_static! {
    static ref V: Vec<i32> = cache();
}

fn cache() -> Vec<i32> {
    let mut tt = Vec::with_capacity(5611771);
    for d in 0..=9 {
        let bit = 1 << d;
        let curnum = d;
        let mask = if curnum == 0 { 0 } else { bit };

        for d in 0..=9 {
            let bit = 1 << d;
            if (curnum != 0 || d != 0) && mask & bit != 0 {
                continue;
            }
            let curnum = 10 * curnum + d;
            let mask = if curnum == 0 { mask } else { mask | bit };

            for d in 0..=9 {
                let bit = 1 << d;
                if (curnum != 0 || d != 0) && mask & bit != 0 {
                    continue;
                }
                let curnum = 10 * curnum + d;
                let mask = if curnum == 0 { mask } else { mask | bit };

                for d in 0..=9 {
                    let bit = 1 << d;
                    if (curnum != 0 || d != 0) && mask & bit != 0 {
                        continue;
                    }
                    let curnum = 10 * curnum + d;
                    let mask = if curnum == 0 { mask } else { mask | bit };
                    for d in 0..=9 {
                        let bit = 1 << d;
                        if (curnum != 0 || d != 0) && mask & bit != 0 {
                            continue;
                        }
                        let curnum = 10 * curnum + d;
                        let mask = if curnum == 0 { mask } else { mask | bit };
                        for d in 0..=9 {
                            let bit = 1 << d;
                            if (curnum != 0 || d != 0) && mask & bit != 0 {
                                continue;
                            }
                            let curnum = 10 * curnum + d;
                            let mask = if curnum == 0 { mask } else { mask | bit };
                            for d in 0..=9 {
                                let bit = 1 << d;
                                if (curnum != 0 || d != 0) && mask & bit != 0 {
                                    continue;
                                }
                                let curnum = 10 * curnum + d;
                                let mask = if curnum == 0 { mask } else { mask | bit };
                                for d in 0..=9 {
                                    let bit = 1 << d;
                                    if (curnum != 0 || d != 0) && mask & bit != 0 {
                                        continue;
                                    }
                                    let curnum = 10 * curnum + d;
                                    let mask = if curnum == 0 { mask } else { mask | bit };
                                    for d in 0..=9 {
                                        let bit = 1 << d;
                                        if (curnum != 0 || d != 0) && mask & bit != 0 {
                                            continue;
                                        }
                                        let curnum = 10 * curnum + d;

                                        tt.push(curnum);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    tt
}