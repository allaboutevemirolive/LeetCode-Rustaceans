// https://leetcode.com/problems/create-maximum-number/solutions/3396659/share-my-rust-code/
// pub struct Solution;

impl Solution {
    /* 从数组中选出k个数，使他最大。 */
    pub fn helper(nums: & Vec<i32>, k: usize) -> Vec<i32> {
        let mut stk: Vec<i32> = Vec::new();
        if k == 0 {
            return stk;
        }
        let mut size = k; /* 栈剩余的容量 */
        let mut left = nums.len(); /* 剩余的数字 */
        for v in nums {
            let v = *v;
            /* 容量刚好填满时，all in。 */
            if size == left || stk.is_empty() {
                stk.push(v);
                size -= 1;
                left -= 1;
                continue;
            }
            let mut top = *stk.last().unwrap();
            if top >= v && size == 0 {
                left -= 1;
                continue;
            }
            if top >= v && size > 0 {
                stk.push(v);
                size -= 1;
                left -= 1;
                continue;
            }
            /* 只要还能填满，并且栈顶的值比当前值要小，就一直出栈。 */
            while size < left && !stk.is_empty() {
                top = *stk.last().unwrap();
                if top < v {
                    stk.pop();
                    size += 1;
                } else {
                    break;
                }
            }
            stk.push(v);
            size -= 1;
            left -= 1;
        }
        stk
    }
    /* 把两个数组组合起来找最大的值 */
    fn combine(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut i = 0;
        let mut k = 0;
        let mut res = Vec::new();
        while i < nums1.len() && k < nums2.len() {
            /* 谁大选谁 */
            if nums1[i] < nums2[k] {
                res.push(nums2[k]);
                k += 1;
            } else if nums1[i] > nums2[k] {
                res.push(nums1[i]);
                i += 1;
            } else {
                /* 如果候选人一样的话，就有些难处理了。
                 * 需要挨个看后面的值谁大，选大的。 */
                let ori_i = i;
                let ori_k = k;
                i += 1;
                k += 1;
                let mut use_i = false;
                while i < nums1.len() && k < nums2.len() {
                    if nums1[i] < nums2[k] {
                        use_i = false;
                        break;
                    } else if nums1[i] > nums2[k] {
                        use_i = true;
                        break;
                    }
                    i += 1;
                    k += 1;
                }
                /* 比到最后还是一样，选后面还有数的。 */
                if i == nums1.len() && k < nums2.len() {
                    use_i = false;
                } else if i < nums1.len() && k == nums2.len() {
                    use_i = true;
                }
                /* 记得把序号恢复回来。 */
                if use_i {
                    res.push(nums1[ori_i]);
                    i = ori_i + 1;
                    k = ori_k;
                } else {
                    res.push(nums2[ori_k]);
                    i = ori_i;
                    k = ori_k + 1;
                }
            }
        }
        while i < nums1.len() {
            res.push(nums1[i]);
            i += 1;
        }
        while k < nums2.len() {
            res.push(nums2[k]);
            k += 1;
        }
        res
    }
    /* 比较两个数组哪个大 */
    fn nums1_larger(nums1: & Vec<i32>, nums2: & Vec<i32>) -> bool {
        let length = nums1.len();
        for i in 0..length {
            if nums1[i] > nums2[i] {
                return true;
            } else if nums1[i] < nums2[i] {
                return false;
            }
        }
        return false;
    }
    pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        let large: Vec<i32>;
        let small: Vec<i32>;
        if nums1.len() > nums2.len() {
            large = nums1;
            small = nums2;
        } else {
            large = nums2;
            small = nums1;
        }
        let start = if k as usize > large.len() {
            k as usize - large.len()
        } else {
            0
        };
        let stop = std::cmp::min(k as usize + 1, small.len() + 1);
        let mut res = vec![0; k as usize];
        for i in start..stop {
            let v1 = Solution::helper(&small, i);
            let v2 = Solution::helper(&large, k as usize - i);
            let cur = Solution::combine(v1, v2);
            if Solution::nums1_larger(&cur, &res) {
                res = cur;
            }
        }
        res
    }
}