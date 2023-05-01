// https://leetcode.com/problems/kth-largest-element-in-an-array/solutions/3333561/rust-approaches/
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        fn using_sorting(mut nums: Vec<i32>, k: i32) -> i32 {
            nums.sort();
            nums[nums.len() - k as usize]
        }
        fn using_count_sort(nums: Vec<i32>, mut k: i32) -> i32 {
            const PAD: usize = 10000;
            let mut arr = vec![0; 2 * PAD + 1];
            for num in nums {
                arr[num as usize + PAD] += 1;
            }
            let mut i = arr.len() - 1;
            while k > 0 {
                if arr[i] == 0 {
                    i -= 1;
                    continue;
                } else {
                    let to_delete = arr[i].min(k);
                    arr[i] -= to_delete;
                    k -= to_delete;
                }
            }
            i as i32 - PAD as i32
        }
        using_count_sort(nums, k)  
    }
}