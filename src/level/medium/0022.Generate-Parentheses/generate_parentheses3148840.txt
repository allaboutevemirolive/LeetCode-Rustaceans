// https://leetcode.com/problems/generate-parentheses/solutions/3148840/rust-solution/
impl Solution {
    pub fn rec(mut n : i32,mut ans : &mut Vec<String>,mut strr : &mut String,mut open : &mut i32) {
        if n == 0 {
            for i in 0..*open {
                strr.push(')');
            }
            ans.push(strr.to_string());
            for i in 0..*open {
                strr.pop();
            }
            return;
        }

        if n > 0 {
            strr.push('(');
            Self::rec(n-1, ans, strr,&mut(*open + 1));
            strr.pop();
        }

        if open > &mut 0 {
            strr.push(')');
            Self::rec(n, ans, strr,&mut(*open-1));
            strr.pop();
        }

    }
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ans : Vec<String> = vec![];
        let mut strr : String = String::new();
        let mut open : i32 = 0;
        Self::rec(n,&mut ans,&mut strr,&mut open);
        ans
    }
}