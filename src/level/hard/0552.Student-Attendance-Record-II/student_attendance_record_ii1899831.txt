// https://leetcode.com/problems/student-attendance-record-ii/solutions/1899831/rust-50ms-faster-than-100-in-time-and-memory-o-n-time-and-o-1-extra-space-with-explanation/
macro_rules! add_with_mod {
    ( $( $x:expr ),* ) => {
        {
            let mut ans = 0i32;
            let r#mod = 1e9 as i32 + 7;
            $(
                ans += $x;
                if ans >= r#mod {
                    ans -= r#mod;
                }
            )*
            ans
        }
    };
}

impl Solution {
    pub fn check_record(n: i32) -> i32 {
        if n == 1 {
            return 3;
        }
        let n = n as usize;

        // f[i][totalA][A], f[i][totalA][L], f[i][totalA][P]
		// A => 0, L => 1, P => 2
        // f denotes the total number of valid strings considering i places (`i` length string)
        // having total number of A's  = totalA which can be either 0 or 1
        // and the last dimension specifies the letter we are choosing in current i mean ith position
        // based on the logic we will only need to care about last two states to evaluate current state
        // so using space optimization here
        // f[i][0][0] implies count of valid strings ending with 'A' in current position (`i`)
        // having totalA count = 0
        let mut f = vec![vec![vec![0; 3]; 2]; 3];
        f[0] = vec![vec![0, 1, 1], vec![1, 0, 0]];
        f[1] = vec![vec![0, 2, 2], vec![2, 1, 1]];
        for _ in 2..n {
            // put a ,total 1 a
            // placed 'A' in current position, so we can extend all strings ending with
            // 'P' or 'L' in last position with 0 total A's
            // `add_with_mod` is a declarative macro to avoid repeatable code and replace `%` operator with `-` operator
            f[2][1][0] = add_with_mod!(f[1][0][1], f[1][0][2]);
            // put l total 1 a
            // placed 'L' in current position, and we need 1 'A' in total
            // so we can extend all strings ending with A in last position
            f[2][1][1] = f[1][1][0];
            // or if char in last position is 'L' then we can extend only
            // strings ending with 'P' or 'A' in second last position as we want to avoid
            // 3 consecutive L's
            f[2][1][1] = add_with_mod!(f[2][1][1], f[0][1][0], f[0][1][2]);
            // or if char in last position is 'P' then second last position can be any of the 3 characters
            // but it should have exactly  1 'A'
            f[2][1][1] = add_with_mod!(f[2][1][1], f[0][1][0], f[0][1][2], f[0][1][1]);
            // put p total 1a
            // similarly no restrictions if we place 'P' in current position
            f[2][1][2] = add_with_mod!(f[1][1][0], f[1][1][1], f[1][1][2]);
            // put l total 0a
            // placed 'L' in current position and totalA count = 0, so only valid chars are 'P' and 'L'
            // so char in last position can be 'P' and we have no problem
            // if char in last position is 'L' then we can only have 'P' as second last character,
            // as we want to avoid 3 or more consecutive L's
            f[2][0][1] = add_with_mod!(f[1][0][2], f[0][0][2]);
            // put p total 0a
            // similarly if current char is 'P', then we don't care about anything else apart from
            // the totalA count we want to maintain
            f[2][0][2] = add_with_mod!(f[1][0][0], f[1][0][1], f[1][0][2]);
            // swapping to compute next state 0, 1 belong to (i - 2) th and (i - 1) th state respectively, 2 represents ith state
            f.swap(0, 1);
            f.swap(1, 2);
        }
        // println!("{:?}", f[n - 1]);
        // answer is cumulative sum of all possible states in f[1]
        let mut ans = 0;
        for j in 0..2 {
            for k in 0..3 {
                ans = add_with_mod!(ans, f[1][j][k]);
            }
        }
        ans
    }
}