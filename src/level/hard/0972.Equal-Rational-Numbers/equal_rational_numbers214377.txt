// https://leetcode.com/problems/equal-rational-numbers/solutions/214377/rust-no-cheat-solution-with-explanation/
use std::ops::Add;

// Define rational type to represent the number. First field is numerator. Second is denominator.
struct Rational(i32, i32);

impl Rational {
    // Factory method to create Rational from String.
    fn parse_from(s: String) -> Rational {
        let (int, dot_pos) = parse_int(&s);
        if dot_pos + 1 >= s.len() {
            return int;
        }
        let (nonrepeat, paren_pos) = parse_nonrepeating(&s, dot_pos);
        if paren_pos + 1 >= s.len() {
            return int + nonrepeat;
        }
        let repeat = parse_repeating(&s, dot_pos, paren_pos);
        repeat + int + nonrepeat
    }
}

// Implement Add trait so that we can write Rational + Rational.
impl Add for Rational {
    type Output = Rational;

    fn add(self, other: Rational) -> Rational {
        Rational(self.0 * other.1 + self.1 * other.0, self.1 * other.1)
    }
}

// Implement PartialEq trait so that we can compare two Rationals with ==. We cannot use
// #[derive(PartialEq)] as it will only do a field-wise comparison.
impl PartialEq for Rational {
    fn eq(&self, other: &Rational) -> bool {
        self.0 * other.1 == self.1 * other.0
    }
}

// Top-level is_rational_equal function.
fn is_rational_equal(s: String, t: String) -> bool {
    Rational::parse_from(s) == Rational::parse_from(t)
}

// Parse integer part before decimal point. Parsing function returns the position
// of "continuation" as where to start next parsing.
fn parse_int(s: &str) -> (Rational, usize) {
    match s.find(".") {
        None => (Rational(s.parse().unwrap(), 1), s.len()),
        Some(p) => (Rational(s[..p].parse().unwrap_or(0), 1), p)
    }
}

// Parse nonrepeating part. For edge case like "1.", we return (0, 1).
fn parse_nonrepeating(s: &str, dot_pos: usize) -> (Rational, usize) {
    let paren_pos = s.find("(").unwrap_or(s.len());
    (Rational(s[dot_pos+1..paren_pos].parse().unwrap_or(0), i32::pow(10, (paren_pos - dot_pos - 1) as u32)), paren_pos)
}

// Parse repeating part. This is the last part so don't need to return continuation position. Also the last character must be
// ")". So I just use s.len() - 1 as its position.
fn parse_repeating(s: &str, dot_pos: usize, paren_pos: usize) -> Rational {
    let pow = i32::pow(10, (paren_pos - dot_pos - 1) as u32);
    let num_nines = s.len() - 1 - paren_pos - 1;
	// Generate string like "9999..9" -- the number of 9s equal to the length of the repeating part 
	// and cast it to integer. We can be more efficient by using reduce operator but it's just overkill. 
    let nines = std::iter::repeat("9").take(num_nines).collect::<String>().parse::<i32>().unwrap();
    Rational(s[paren_pos+1..s.len()-1].parse().unwrap(), nines * pow)
}

impl Solution {
    pub fn is_rational_equal(s: String, t: String) -> bool {
        is_rational_equal(s, t)
    }
}