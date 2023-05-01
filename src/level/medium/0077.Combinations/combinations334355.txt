// https://leetcode.com/problems/combinations/solutions/334355/8ms-rust-traverse-indices/
pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut result = vec![];
    traverse(0, n, k, &mut (0..k).collect(), &mut result);
    result
}

fn traverse(i: i32, n: i32, k: i32, indices: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    if i == k {
        result.push(indices.iter().map(|&j| j + 1).collect());
    } else {
        // n - indices[i] is the number of rest numbers
        // k - i is the number of numbers still needed to be chosen
        while n - indices[i as usize] >= k - i {
            traverse(i + 1, n, k, indices, result);
            let start = indices[i as usize] + 1;
            for j in i..k {
                indices[j as usize] = start + j - i;
            }
        }
    }
}