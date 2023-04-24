// https://leetcode.com/problems/brace-expansion-ii/solutions/2825626/rust-long-but-easy-to-read-recursive-solution/
use std::collections::BTreeSet;

impl Solution {
    pub fn brace_expansion_ii(expression: String) -> Vec<String> {
        let mut balance = 0;
        let mut data = vec![];
        let mut expression = expression;
        
        Self::remove_brace(&mut expression);
        
        let mut str = String::new(); 
        for c in expression.chars() { 
            if c == '{' { balance += 1; }
            if c == '}' { balance -= 1; }
            
            if c == ',' && balance == 0 {
                if str.is_empty() == false { data.push(str.clone()); }
                str.clear();
            } else { str.push(c); }
        }
        if str.is_empty() == false { data.push(str); }
        
        let mut st = BTreeSet::new();
        for mut s in data {
            let v = Self::process(&mut s);
            for it in v { st.insert(it); }
        }
        
        st.into_iter().collect()
    }
    
    fn process(s: &mut String) -> Vec<String> {
        let mut ret = vec![String::new()];
        
        let mut str = String::new();
        let mut balance = 0;
        
        for c in s.chars() {
            if c == '{' {
                balance += 1;
                
                if balance == 1 {
                    for i in 0 .. ret.len() { ret[i] += &str; }
                    str.clear();
                } else { str.push(c); }
                continue
            }
            
            if c == '}' {
                balance -= 1;
                
                if balance == 0 {
                    if str.is_empty() { continue }
                    let data = Self::brace_expansion_ii(str.clone());
                    let mut temp = vec![];
                    
                    for a in &ret {
                        for it in &data {
                            let mut it1 = (*a).clone();
                            it1 += it;
                            temp.push(it1);
                        }
                    }
                    ret = temp;
                    str.clear();
                } else { str.push(c); }
                continue
            }
            
            str.push(c);
        }
        
        for i in 0 .. ret.len() { ret[i] += &str; }
        
        ret
    }
    
    fn remove_brace(s: &mut String) {
        let n = s.len();
        let mut t = s.chars().collect::<Vec<char>>();
        if t[0] != '{' || t[n - 1] != '}' { return }
        
        let mut balance = 0;
        for i in 0 .. n {
            if t[i] == '{' { balance += 1; }
            if t[i] == '}' { balance -= 1; }
            if i > 0 && i + 1 < n && balance == 0 { return }
        }
        
        t.pop();
        t.reverse();
        t.pop();
        t.reverse();
       *s = t.iter().collect(); 
    }
}