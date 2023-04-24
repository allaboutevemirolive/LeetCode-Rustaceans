// https://leetcode.com/problems/palindrome-partitioning-iii/solutions/2931463/rust-simple-solution-recursion-memorization/
use std::collections::HashMap;
impl Solution {
    pub fn palindrome_partition(s: String, k: i32) -> i32 {
        let c:Vec<char> = s.chars().collect();
        fn dfs(k: i32, l: usize, c: &Vec<char>, memo: &mut HashMap<(i32,usize),i32>) -> i32 {
            if k as usize >= l { return 0; }
            if k == 0 && l > 0 {return l as  i32;}
            if let Some(r) = memo.get(&(k,l)) {
                return *r;
            }
            let mut ret = l as i32;
            for ll in ((k-1) as usize..l) {
                let mut ri  = l-1;
                let mut li  = ll;
                let mut lc = 0; //steps to replace the right part to palindrome
                while li < ri {
                    if c[li] != c[ri] { lc += 1;}
                    li += 1;
                    ri -= 1;
                }
                ret = ret.min(dfs(k-1,ll, c, memo)+ lc);
            }
            memo.insert((k,l), ret);
            //println!("{:?}", memo);
            return ret;
        }

        let mut memo: HashMap<(i32,usize),i32> = HashMap::new();
        return dfs(k, c.len(), &c, &mut memo);
    }
}