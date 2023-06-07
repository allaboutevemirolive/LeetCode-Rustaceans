// https://leetcode.com/problems/parsing-a-boolean-expression/solutions/1764562/rust-recursion/
enum Op {
    And,
    Or,
}

impl Solution {
    pub fn expr_list(expr: &[u8], idx: &mut usize, op: Op, accum: bool) -> bool {
        if expr[*idx] == b',' {
            *idx+=1;
        }
        
        if expr[*idx]==b')' {
            return accum;    
        }
        
        let ret = Self::inner(expr, idx);
        let combine = match op {
            Op::And => ret && accum,
            _ => ret || accum
        };
        Self::expr_list(expr,idx,op,combine)
    }
    pub fn inner(expr: &[u8], idx: &mut usize) -> bool {
        let c = expr[*idx];
        match c {
            b't' => {
                *idx+=1;
                true
            },
            b'f' => {
                *idx+=1;
                false
            },
            b'!' => {
                *idx+=2;
                let ret = !Self::inner(expr,idx);
                *idx+=1;
                ret
            },
            b'&' => {
                *idx+=2;
                let ret = Self::expr_list(expr, idx, Op::And, true);
                *idx+=1;
                ret
            },
            b'|' => {
                *idx+=2;
                let ret = Self::expr_list(expr, idx, Op::Or, false);
                *idx+=1;
                ret
            },
            _ => panic!("unexpected character: {}", c)
        }
    }
    
    pub fn parse_bool_expr(expression: String) -> bool {
        let mut idx = 0;
        Self::inner(expression.as_str().as_bytes(), & mut idx)
    }
}