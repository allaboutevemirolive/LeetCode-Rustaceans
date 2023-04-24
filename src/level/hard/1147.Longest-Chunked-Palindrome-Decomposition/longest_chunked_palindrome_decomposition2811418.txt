// https://leetcode.com/problems/longest-chunked-palindrome-decomposition/solutions/2811418/fastest-and-also-rather-straight-forward-rust-solution-by-matching-substrings-o-n/
impl Solution {
    pub fn longest_decomposition(text: String) -> i32 {
        let mut tokencount = 0;
        let mut prevtokenend = 0;
        let tl = text.len();
        let maxi = (tl)/2+1;
        for i in 1..maxi{
            if (i<=prevtokenend){continue;} //length of token would be 0, skip
            if text[prevtokenend..i] == text[(tl-i)..(tl-prevtokenend)]{
                //the token matched, count it and set start for next token to current end
                tokencount +=1;
                prevtokenend = i;
            }
        }
        tokencount
    }
}