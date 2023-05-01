// https://leetcode.com/problems/find-peak-element/solutions/3135063/rust-o-logn-clean/
impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let (mut start, mut end) = (0, nums.len());
        while start < end {
            let mid = (start + end)/2;
            let c = cmpIdx(&nums[..], mid);
            // println!("index: {} case: {:?}", mid, c);
            match c {
                Case::Peak => return mid as i32,
                Case::Inc | Case::Valley => start = mid + 1,
                Case::Dec => end = mid,
            };
        }
       0 
    }
}

#[inline]
fn cmpIdx(a: &[i32], idx: usize) -> Case {
    // true if idx greater than idx-1
    let gtl = if idx == 0 { true } else { a[idx] > a[idx-1] };
    let gtr = if idx == a.len() - 1 { true } else { a[idx] > a[idx+1] };
    if gtl && gtr {
        Case::Peak
    } else if !gtl && !gtr {
        Case::Valley
    } else if gtl && !gtr {
        Case::Inc
    } else {
        Case::Dec
    }
}

#[derive(Debug)]
enum Case {
    Peak,
    Valley,
    Inc,
    Dec,
}