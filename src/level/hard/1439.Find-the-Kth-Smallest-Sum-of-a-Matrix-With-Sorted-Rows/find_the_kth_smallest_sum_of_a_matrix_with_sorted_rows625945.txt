// https://leetcode.com/problems/find-the-kth-smallest-sum-of-a-matrix-with-sorted-rows/solutions/625945/rust-9-lines-simple-solution-12ms/
impl Solution {
    pub fn kth_smallest(mat: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as usize; 
        let mut cc:Vec<i32> = mat[0].to_vec();
		
        for row in mat.iter().skip(1){
            let mut ncc = row.iter().map(|p| cc.iter().map(move |q| p + q)).flatten().collect::<Vec<i32>>();
            ncc.sort_unstable(); 
            cc = ncc.into_iter().take(k).collect();
        }
       cc.last().unwrap()
    }
}