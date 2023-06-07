// https://leetcode.com/problems/minimum-number-of-flips-to-convert-binary-matrix-to-zero-matrix/solutions/2745992/rust-100-easy-dfs-helper-functions/
type Matrix = Vec<Vec<bool>>;

pub fn is_zero(mat: &Matrix) -> bool {
    mat.iter().all(|row| row.iter().all(|x| !x))
}

pub fn flip(position: i32, mat: &mut Matrix) {
    let x = (position / (mat.len() as i32)) as usize;
    let y = (position % (mat.len() as i32)) as usize;
    mat[y][x] = !mat[y][x];
    if y >= 1 {
        mat[y - 1][x] = !mat[y - 1][x];
    }
    if y < mat.len() - 1 {
        mat[y + 1][x] = !mat[y + 1][x];
    }
    if x >= 1 {
        mat[y][x - 1] = !mat[y][x - 1];
    }
    if x < mat[0].len() - 1 {
        mat[y][x + 1] = !mat[y][x + 1];
    }
}

pub fn _min_flips(position: i32, mat: &mut Matrix) -> Option<i32> {
    // edge case
    if position as usize == mat.len() * mat[0].len() {
        if is_zero(mat) {
            return Some(0);
        } else {
            return None;
        }
    }

    let min: Option<i32> = None;

    // don't flip this one
    let result_0 = _min_flips(position + 1, mat);
    // flip the position
    flip(position, mat);
    let result_1 = _min_flips(position + 1, mat);
    // flip back
    flip(position, mat);

    if result_0.is_some() && result_1.is_some() {
        return Some(result_0.unwrap().min(1+result_1.unwrap()));
    } else if result_0.is_some() {
        return result_0;
    } else if result_1.is_some() {
        return Some(1+result_1.unwrap());
    }

    None
}

pub fn min_flips(mat: Vec<Vec<i32>>) -> i32 {
    let mut mat = &mut mat
        .iter()
        .map(|row| row.iter().map(|x| *x == 1).collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>();

    if let Some(x) = _min_flips(0, &mut mat) {
        return x;
    } else {
        return -1;
    }
}