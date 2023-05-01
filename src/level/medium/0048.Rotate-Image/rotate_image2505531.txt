// https://leetcode.com/problems/rotate-image/solutions/2505531/rust-simple-solution/
impl Solution {
  pub fn rotate(mat: &mut Vec<Vec<i32>>) {
    let len = mat.len() - 1;
    let half = mat.len() / 2;

    for i in (0..half) {
      for j in (0..(half+mat.len()%2)) {
        let mat_____i______j = mat[i][j];
        
        mat[    i][    j] = mat[len-j][i    ];
        mat[len-j][    i] = mat[len-i][len-j];
        mat[len-i][len-j] = mat[    j][len-i];
        mat[    j][len-i] = mat_____i______j;
      }
    }
  }
}