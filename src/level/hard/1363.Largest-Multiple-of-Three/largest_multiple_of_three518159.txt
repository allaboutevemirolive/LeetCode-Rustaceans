// https://leetcode.com/problems/largest-multiple-of-three/solutions/518159/rust-bucket-sort-solution/

impl Solution {
    pub fn largest_multiple_of_three(digits: Vec<i32>) -> String {
        let mut bucket: [usize;10] = [0;10];
        let mut res: Vec<i32> = Vec::new();
        for d in digits.iter() {
            bucket[*d as usize] += 1;
        }
        let (mut cnt_g1, mut cnt_g2) = (0 as usize,0 as usize);
        for i in 0..10 {
            if i == 0 || i == 3 || i == 6 || i == 9 {
                for _ in 0..bucket[i as usize] {
                    res.push(i);
                }
                bucket[i as usize] = 0;
            } else if i == 1 || i == 4 || i == 7 {
                cnt_g1 += bucket[i as usize];
            } else {
                cnt_g2 += bucket[i as usize];
            }
        }
        // println!("{:?}, g1: {}, g2 {}", bucket,cnt_g1%3, cnt_g2%3);
        if cnt_g1 % 3 == cnt_g2 % 3 {
            let mut fin_res = digits;
            fin_res.sort_by(|a, b| {
                b.cmp(a)
            });
            if fin_res[0] == 0 {
                return String::from("0");
            }
            return fin_res.iter().map(|a| {((*a + '0' as i32) as u8) as char}).collect::<String>();
        }
        if cnt_g1 > 0 && cnt_g1 % 3 == 0 && cnt_g2 % 3 == 2 {
            for _ in 0..(cnt_g1 - 1) {
                if bucket[7] > 0 {
                    res.push(7);
                    bucket[7] -= 1;
                } else if bucket[4] > 0 {
                    res.push(4);
                    bucket[4] -= 1;
                } else if bucket[1] > 0 {
                    res.push(1);
                    bucket[1] -= 1;
                }
            }
            for _ in 0..(cnt_g2) {
                if bucket[8] > 0 {
                    res.push(8);
                    bucket[8] -= 1;
                } else if bucket[5] > 0 {
                    res.push(5);
                    bucket[5] -= 1;
                } else if bucket[2] > 0 {
                    res.push(2);
                    bucket[2] -= 1;
                }
            }
             
        } else if cnt_g2 > 0 && cnt_g2 % 3 == 0 && cnt_g1 % 3 == 2 {
            for _ in 0..(cnt_g2 - 1) {
                if bucket[8] > 0 {
                    res.push(8);
                    bucket[8] -= 1;
                } else if bucket[5] > 0 {
                    res.push(5);
                    bucket[5] -= 1;
                } else if bucket[2] > 0 {
                    res.push(2);
                    bucket[2] -= 1;
                }
            }
            for _ in 0..(cnt_g1) {
                if bucket[7] > 0 {
                    res.push(7);
                    bucket[7] -= 1;
                } else if bucket[4] > 0 {
                    res.push(4);
                    bucket[4] -= 1;
                } else if bucket[1] > 0 {
                    res.push(1);
                    bucket[1] -= 1;
                }
            }
             
        } else {
            for _ in 0..(cnt_g1 - cnt_g1 % 3) {
                if bucket[7] > 0 {
                    res.push(7);
                    bucket[7] -= 1;
                } else if bucket[4] > 0 {
                    res.push(4);
                    bucket[4] -= 1;
                } else if bucket[1] > 0 {
                    res.push(1);
                    bucket[1] -= 1;
                }
            }
            cnt_g1 = cnt_g1 % 3;
            for _ in 0..(cnt_g2 - cnt_g2 % 3) {
                if bucket[8] > 0 {
                    res.push(8);
                    bucket[8] -= 1;
                } else if bucket[5] > 0 {
                    res.push(5);
                    bucket[5] -= 1;
                } else if bucket[2] > 0 {
                    res.push(2);
                    bucket[2] -= 1;
                }
            }
            cnt_g2 = cnt_g2 % 3;
            if cnt_g1 + cnt_g2 == 3 {
                if bucket[7] > 0 {
                    res.push(7);
                    bucket[7] -= 1;
                } else if bucket[4] > 0 {
                    res.push(4);
                    bucket[4] -= 1;
                } else if bucket[1] > 0 {
                    res.push(1);
                    bucket[1] -= 1;
                }
                if bucket[8] > 0 {
                    res.push(8);
                    bucket[8] -= 1;
                } else if bucket[5] > 0 {
                    res.push(5);
                    bucket[5] -= 1;
                } else if bucket[2] > 0 {
                    res.push(2);
                    bucket[2] -= 1;
                }
            }
        }
        res.sort_by(|a,b| {b.cmp(a)});
        res.iter().map(|a| {((*a + '0' as i32) as u8) as char}).collect::<String>()
    }
}
