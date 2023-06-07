// https://leetcode.com/problems/stone-game-iii/solutions/3566375/rust-dp-o-n/
struct DPImpl {
    stone_value: Vec<i32>,

    suffix_sum_vec: Vec<i32>,

    cache_vec: Vec<Option<i32>>,
}

impl DPImpl {
    fn gen_suffix_sums(stone_value: &Vec<i32>) -> Vec<i32> {
        let mut val_vec = vec![0; stone_value.len() + 1];
        for i in (0..stone_value.len()).rev() {
            val_vec[i] = val_vec[i + 1] + stone_value[i];
        }
        val_vec
    }

    pub fn new(stone_value: Vec<i32>) -> Self {
        let suffix_sum_vec = Self::gen_suffix_sums(&stone_value);

        let cache_vec = vec![None; stone_value.len() + 1];

        Self {
            stone_value,

            suffix_sum_vec,
            cache_vec,
        }
    }

    pub fn get(&mut self, i: usize) -> Option<i32> {
        if i > self.stone_value.len() {
            return None; // out-of-bound
        }

        if i == self.stone_value.len() {
            return Some(0); // nothing left
        }

        {
            let value_opt = if let Some(data) = self.cache_vec.get(i) {
                data.clone()
            } else {
                return None; // out of bound somehow, panic?
            };

            if let Some(value) = value_opt {
                return Some(value); // cached
            }
        }

        let mut val_max_opt: Option<i32> = None;

        for j in 0..3 { // 0 <= j <= 2
            let i_next = i + j + 1; // 1 <= j <= 3

            if i_next >= self.cache_vec.len() {
                break;
            }

            if let Some(val_next) = self.get(i_next) {
                let val = self.suffix_sum_vec[i] - val_next;
                if let Some(val_max) = val_max_opt.clone() {
                    val_max_opt = Some(val_max.max(val));
                } else {
                    val_max_opt = Some(val);
                }
            }
        }

        self.cache_vec[i] = val_max_opt;

        val_max_opt
    }

    pub fn total_score(&self) -> i32 {
        self.suffix_sum_vec[0]
    }
}


impl Solution {
    pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
        let mut dp = DPImpl::new(stone_value);
        let val = dp.get(0).unwrap();

        println!("val: {}", val);

        if val * 2 < dp.total_score() {
            "Bob".to_string()
        } else if val * 2 > dp.total_score() {
            "Alice".to_string()
        } else {
            "Tie".to_string()
        }
    }
}