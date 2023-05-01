// https://leetcode.com/problems/longest-palindromic-substring/solutions/255887/rust-solution/
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len()==0{
            return "".to_string()
        }
        let (mut start,mut end)=(0,0);
        let b=s.as_bytes();
        let mut maxlen=-1;
        let slen=s.len() as i32;
        for i in 0..slen as i32{
            let mut tmplen=-1;
            let mut j:i32=-1;
            loop{
                if i-j-1>=0&&i+j+1<slen&&b[(i-j-1) as usize]==b[(i+j+1) as usize]{
                    j+=1;
                    tmplen+=2;
                }else{
                    if maxlen<tmplen{
                        maxlen=tmplen;
                        start=i-j;
                        end=i+j;
                    }
                    break;
                }
            }

            tmplen=0;
            j=-1;
            loop{
                if i-j-1>=0&&i+2+j<slen&&b[(i-j-1) as usize]==b[(i+2+j) as usize]{
                    j+=1;
                    tmplen+=2;
                }else{
                    if maxlen<tmplen{
                        maxlen=tmplen;
                        start=i-j;
                        end=i+j+1;
                    }
                    break;
                }
            }
        }
        (s[start as usize..=end as usize]).to_string()
    }

}