// https://leetcode.com/problems/brace-expansion-ii/solutions/623103/rust-4ms-2-3-mb-a-complex-solution/

use std::collections::BTreeSet;
use std::{error::Error, fmt::Display, str::Chars};

// rules
//   R(x) = { x }
//   R({e1, e2}) = R(e1) U R(e2)
//   R{e1 + e2} = { a + b for (a, b) in R(e1) x R(e2) }

// Expr = concatenate(Items)  = Item Item*
type Expr = Vec<Box<Item>>;
// Item = Word | Union(Exprs)
enum Item {
    // a-z
    Word(String),
    // '{' Expr (',' Expr)* '}'
    UnionList(Vec<Box<Expr>>),
}

impl Solution {
    pub fn brace_expansion_ii(expression: String) -> Vec<String> {
        let expr = Self::parse_expression(&expression).unwrap();
        let set = Self::expand_expression(expr);
        return set;
    }

    fn parse_expression(expression: &String) -> Result<Box<Expr>, ParserError> {
        Parser::new(expression).parse_expr()
    }

    fn expand_expression(expr: Box<Expr>) -> Vec<String> {
        let mut items: Vec<Vec<String>> = vec![];
        for item in expr.into_iter() {
            items.push(Self::expand_item(item));
        }
        let mut buf = vec![];
        let mut result = BTreeSet::new();
        Self::combinate_strings(&items, 0, &mut buf, &mut result);
        result.into_iter().collect()
    }

    fn combinate_strings(
        str_list: &Vec<Vec<String>>,
        idx: usize,
        buf: &mut Vec<String>,
        result: &mut BTreeSet<String>,
    ) {
        if idx >= str_list.len() {
            result.insert(buf.join(""));
            return;
        }
        for s in str_list[idx].iter() {
            buf.push(s.to_owned());
            Self::combinate_strings(str_list, idx + 1, buf, result);
            buf.pop();
        }
    }

    fn expand_item(item: Box<Item>) -> Vec<String> {
        match *item {
            Item::Word(s) => vec![s.into()],
            Item::UnionList(expr_list) => {
                let mut set = BTreeSet::<String>::new();
                for expr in expr_list.into_iter() {
                    let str_list = Self::expand_expression(expr);
                    for s in str_list.into_iter() {
                        set.insert(s);
                    }
                }
                set.into_iter().collect()
            }
        }
    }
}

#[derive(Debug)]
enum ParserError {
    InvalidExpression(String),
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ParserError {}

struct Parser<'a> {
    iter: Chars<'a>,
    buf: Option<char>,
}

impl<'a> Parser<'a> {
    fn new(expression: &'a String) -> Parser<'a> {
        Parser {
            iter: expression.chars(),
            buf: None,
        }
    }

    fn parse_expr(&mut self) -> Result<Box<Expr>, ParserError> {
        // expression starts with '{' or 'characters', ends with ',' or '}'
        let mut expr = Box::<Expr>::new(vec![]);
        loop {
            let item = self.parse_item()?;
            match item {
                None => {
                    break;
                }
                Some(item) => {
                    expr.push(item);
                }
            }
        }
        Ok(expr)
    }

    fn parse_item(&mut self) -> Result<Option<Box<Item>>, ParserError> {
        let c = match self.get_char() {
            None => return Ok(None),
            Some(c) => c,
        };
        match c {
            ',' | '}' => {
                self.unget_char(c);
                return Ok(None);
            }
            '{' => {
                // comma separated union list
                let mut expr_list: Vec<Box<Expr>> = vec![];
                loop {
                    let expr = self.parse_expr()?;
                    expr_list.push(expr);
                    // check end of UnionList
                    match self.get_char() {
                        Some(',') => continue,
                        Some('}') => return Ok(Some(Box::new(Item::UnionList(expr_list)))),
                        None => {
                            return Err(ParserError::InvalidExpression(
                                "Expect '}' for end of UnionList".to_owned(),
                            ))
                        }
                        Some(c) => {
                            return Err(ParserError::InvalidExpression(format!(
                                "Expect '}}' for end of UnionList, but got: {}",
                                c
                            )))
                        }
                    }
                }
            }
            chr => {
                let mut word = String::new();
                word.push(chr);
                // word
                loop {
                    let next = self.get_char();
                    match next {
                        Some(ch) if Self::is_word(ch) => word.push(ch),
                        Some(ch) => {
                            self.unget_char(ch);
                            break;
                        }
                        None => break,
                    }
                }
                return Ok(Some(Box::new(Item::Word(word))));
            }
        }
    }

    fn is_word(ch: char) -> bool {
        match ch {
            '{' | '}' | ',' => false,
            c if c.is_whitespace() => false,
            _ => true,
        }
    }

    fn unget_char(&mut self, c: char) {
        if self.buf.is_some() {
            panic!(
                "should not unget multiple times: buf = {}",
                self.buf.unwrap()
            );
        }
        self.buf = Some(c)
    }

    fn get_char(&mut self) -> Option<char> {
        if let Some(c) = self.buf {
            self.buf = None;
            return Some(c);
        }
        loop {
            match self.iter.next() {
                None => {
                    return None;
                }
                Some(c) if !c.is_whitespace() => {
                    return Some(c);
                }
                Some(_) => {
                    continue;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1096() {
        let expr_list = vec![
            "",
            "{a,b,c}",
            "{{a,b},{b,c}}",
            "abc",
            "{a,b}{c,d}",
            "a{b,c}{d,e}f{g,h}",
            "{{a,b}{d,e},{b,c}}",
            "{{大家好,😄}{a,b}{d,e},{b,c}}",
        ];
        for expr in expr_list.into_iter() {
            let res = Solution::brace_expansion_ii(expr.into());
            println!("'{}' => {:?}", expr, res);
        }
    }
}
