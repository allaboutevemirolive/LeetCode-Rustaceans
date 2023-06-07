// https://leetcode.com/problems/parsing-a-boolean-expression/solutions/947311/rust-simple-60-lines-0ms/
use std::str::Chars;

pub fn parse_bool_expr(expression: String) -> bool {
    fn eat_and(chars: &mut Chars<'_>) -> bool {
        let mut expr = true;

        chars.next();

        loop {
            expr &= eat_expr(chars);

            match chars.next() {
                Some(')') => break,
                Some(',') => continue,
                Some(..) | None => unreachable!(),
            }
        }

        expr
    }

    fn eat_or(chars: &mut Chars<'_>) -> bool {
        let mut expr = false;

        chars.next();

        loop {
            expr |= eat_expr(chars);

            match chars.next() {
                Some(')') => break,
                Some(',') => continue,
                Some(..) | None => unreachable!(),
            }
        }

        expr
    }

    fn eat_not(chars: &mut Chars<'_>) -> bool {
        chars.next();
        let expr = eat_expr(chars);
        chars.next();

        !expr
    }

    fn eat_expr(chars: &mut Chars<'_>) -> bool {
        match chars.next() {
            Some('t') => true,
            Some('f') => false,
            Some('&') => eat_and(chars),
            Some('|') => eat_or(chars),
            Some('!') => eat_not(chars),
            Some(..) | None => unreachable!(),
        }
    }

    eat_expr(&mut expression.chars())
}