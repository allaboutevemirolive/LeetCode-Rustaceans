// https://leetcode.com/problems/kth-largest-element-in-an-array/solutions/2723849/rust-almost-textbook-quickselect/
fn partition(l: usize, r: usize, nums: &mut Vec<i32>, pivot_idx: usize) -> usize {
    // move pivot elem to the last position
    let pivot_num = nums[pivot_idx];
    nums.swap(pivot_idx, r);

    // bring elems smaller than `pivot_num` to the front
    let mut mark = l;
    for i in l..r+1 {
        if nums[i] < pivot_num {
            nums.swap(i, mark);
            mark += 1;
        }
    }
    // move pivot to the front of the smallest nums partition
    nums.swap(mark, r);
    mark
}

fn quickselect(l: usize, r: usize, nums: &mut Vec<i32>, rank: usize) -> i32 {
    if (l == r) {
        return nums[l];
    }

    // selection logic can be modified here
    let mut pivot_idx = l + (r - l) / 2;
    pivot_idx = partition(l, r, nums, pivot_idx);

    if rank == pivot_idx {
        return nums[pivot_idx];
    }
    else if rank < pivot_idx {
        quickselect(l, pivot_idx - 1, nums, rank)
    }
    else {
        quickselect(pivot_idx + 1, r, nums, rank)
    }
}

impl Solution {    
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let N = nums.len();
        let mut nums = nums; // rust things
        quickselect(0, N - 1, &mut nums, N - k as usize)
    }
}