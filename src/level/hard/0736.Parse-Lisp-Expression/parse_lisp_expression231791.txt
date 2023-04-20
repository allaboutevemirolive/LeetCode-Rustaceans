// https://leetcode.com/problems/parse-lisp-expression/solutions/231791/yet-another-recursive-descent-parser-rust-0ms/
impl Solution {
    pub fn evaluate(expression: String) -> i32 {
        let mut tokens = Tokens::new(expression.as_str());
        let mut env = Environment::new();
        eval(&mut tokens, &mut env)
    }
}

use std::collections::HashMap;

fn eval<'a>(tokens: &mut Tokens<'a>, env: &mut Environment<'a>) -> i32 {
    match tokens.next().unwrap() {
        Token::Num(n) => n,
        Token::Var(v) => env.get(v),
        Token::Start => match tokens.next().unwrap() {
            Token::Add => {
                let left = eval(tokens, env);
                let right = eval(tokens, env);
                let res = left + right;
                assert_eq!(Token::End, tokens.next().unwrap());
                res
            }
            Token::Mul => {
                let left = eval(tokens, env);
                let right = eval(tokens, env);
                let res = left * right;
                assert_eq!(Token::End, tokens.next().unwrap());
                res
            }
            Token::Let => {
                let mut vars_to_pop = vec![];
                loop {
                    match tokens.peek().unwrap() {
                        Token::Var(v) => {
                            tokens.next().unwrap();
                            if tokens.peek().unwrap() == Token::End {
                                let res = env.get(v);
                                for var in vars_to_pop {
                                    env.pop(var);
                                }
                                assert_eq!(Token::End, tokens.next().unwrap());
                                return res;
                            } else {
                                let val = eval(tokens, env);
                                env.put(v, val);
                                vars_to_pop.push(v);
                            }
                        }
                        _ => {
                            let res = eval(tokens, env);
                            for var in vars_to_pop {
                                env.pop(var);
                            }
                            assert_eq!(Token::End, tokens.next().unwrap());
                            return res;
                        }
                    }
                }
            }
            _ => panic!("Invalid expr after '('"),
        },
        _ => panic!("Invalid start of expression"),
    }
}

struct Tokens<'a> {
    expr: &'a [u8],
    start: usize,
    end: usize,
}

#[derive(Debug, PartialEq, Eq)]
enum Token<'a> {
    Start,
    End,
    Add,
    Mul,
    Let,
    Num(i32),
    Var(&'a [u8]),
}

impl<'a> Tokens<'a> {
    fn new(expr: &'a str) -> Self {
        Tokens {
            expr: expr.as_bytes(),
            start: 0,
            end: 0,
        }
    }

    fn peek(&mut self) -> Option<Token<'a>> {
        let start = self.start;
        let end = self.end;
        let res = self.next();
        self.start = start;
        self.end = end;
        res
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.end == self.expr.len() {
            return None;
        }
        while self.expr[self.end] == b' ' {
            self.end += 1;
            if self.end == self.expr.len() {
                return None;
            }
        }
        self.start = self.end;
        self.end += 1;
        match self.expr[self.start] {
            b'(' => return Some(Token::Start),
            b')' => return Some(Token::End),
            _ => {}
        }
        while !(self.expr[self.end] == b' ' || self.expr[self.end] == b')') {
            self.end += 1;
            if self.end == self.expr.len() {
                return None;
            }
        }
        if self.expr[self.start] == b'-'
            || (self.expr[self.start] >= b'0' && self.expr[self.start] <= b'9')
        {
            let as_str = unsafe { std::str::from_utf8_unchecked(&self.expr[self.start..self.end]) };
            return Some(Token::Num(as_str.parse::<i32>().unwrap()));
        }
        let token = match &self.expr[self.start..self.end] {
            b"mult" => Token::Mul,
            b"add" => Token::Add,
            b"let" => Token::Let,
            var => Token::Var(var),
        };
        Some(token)
    }
}

struct Environment<'a> {
    vars: HashMap<&'a [u8], Vec<i32>>,
}

impl<'a> Environment<'a> {
    fn new() -> Self {
        Environment {
            vars: HashMap::new(),
        }
    }

    fn get(&self, var: &[u8]) -> i32 {
        self.vars.get(var).map(|v| v[v.len() - 1]).unwrap()
    }

    fn put(&mut self, var: &'a [u8], val: i32) {
        let entry = self.vars.entry(var).or_insert(vec![]);
        (*entry).push(val);
    }

    fn pop(&mut self, var: &[u8]) {
        self.vars.get_mut(var).map(|v| v.pop().unwrap()).unwrap();
    }
}
#[test]
fn tokenize() {
    let expr = "(let x (add 1 2) y2 (mult 4 7))";
    let tokens = Tokens::new(expr);
    assert_eq!(
        tokens.collect::<Vec<_>>(),
        vec![
            Token::Start,
            Token::Let,
            Token::Var(b"x"),
            Token::Start,
            Token::Add,
            Token::Num(1),
            Token::Num(2),
            Token::End,
            Token::Var(b"y2"),
            Token::Start,
            Token::Mul,
            Token::Num(4),
            Token::Num(7),
            Token::End,
            Token::End,
        ]
    );
}

#[test]
fn test_add() {
    assert_eq!(3, Solution::evaluate("(add 1 2)".into()));
}

#[test]
fn test_mul() {
    assert_eq!(15, Solution::evaluate("(mult 3 (add 2 3))".into()));
}

#[test]
fn test_let() {
    assert_eq!(10, Solution::evaluate("(let x 2 (mult x 5))".into()));
}

#[test]
fn test_let_scope() {
    assert_eq!(
        14,
        Solution::evaluate("(let x 2 (mult x (let x 3 y 4 (add x y))))".into())
    );
}

#[test]
fn test_let_scope_2() {
    assert_eq!(
        6,
        Solution::evaluate("(let x 2 (add (let x 3 (let x 4 x)) x))".into())
    );
}

#[test]
fn test_let_linear() {
    assert_eq!(2, Solution::evaluate("(let x 3 x 2 x)".into()));
}

#[test]
fn test_hard() {
    assert_eq!(
        -8055,
        Solution::evaluate("(let a -122 b 0 (add (add 1 -4) (mult a 66)))".into())
    );
}