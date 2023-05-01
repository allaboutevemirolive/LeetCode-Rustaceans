// https://leetcode.com/problems/evaluate-reverse-polish-notation/solutions/1880334/rust-solution/
use std::collections::HashMap;
type Eval = fn(l: i32, r: i32) -> i32;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut V: Vec<i32> = Vec::new();
        
        let H: HashMap<String, Eval> = HashMap::from([
            ("+".to_string(), |l: i32, r: i32| -> i32 { return l + r; } as Eval),
            ("-".to_string(), |l: i32, r: i32| -> i32 { return l - r; } as Eval),
            ("*".to_string(), |l: i32, r: i32| -> i32 { return l * r; } as Eval),
            ("/".to_string(), |l: i32, r: i32| -> i32 { return l / r; } as Eval),
        ]);
        for t in tokens {
            match t.parse::<i32>() {
                Ok(n) => V.push(n),
                Err(e) => {
                    if let Some(f) = H.get(&t) {
                        let r = V.pop().unwrap();
                        let l = V.pop().unwrap();
                        V.push(f(l, r));
                    }
                }
            }
        }
        return V[0];
    }
}