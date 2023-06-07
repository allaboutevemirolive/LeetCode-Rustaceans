// https://leetcode.com/problems/odd-even-jump/solutions/2906893/rust-solution-beats-80-time/
#[derive(Debug, Clone, Copy)]
struct Trck {
        val: i32,
        idx: usize,
        o_j_i: usize,
        e_j_i: usize,
        o_s:bool,
        e_s:bool
}

#[derive(Debug)]
struct Vidx {
    val: i32,
    idx: usize,
}

fn process_elt(trk: &mut Vec<Trck>,
                tail: &mut Vec<Vidx>, 
                idx: usize) 
{
    let len = trk.len();
    let mut new_trk = trk[idx];

    // base condition
    if (idx == len - 1) {
        new_trk.o_j_i = len-1;
        new_trk.e_j_i = len-1;
        new_trk.o_s = true;
        new_trk.e_s = true;
        trk[idx] = new_trk;
        tail.push(Vidx { val : new_trk.val, idx});
        //println!("Processed {new_trk:?} {tail:?}");
        //println!("======");
        return
    }

    // odd 
    let o_j_e =tail.partition_point(|elt| elt.val < new_trk.val);
    //println!("      o_j_e {o_j_e}");
    if o_j_e < tail.len() {
        new_trk.o_j_i = tail[o_j_e].idx;
        new_trk.o_s = trk[new_trk.o_j_i].e_s;
        //println!("      Hit  odd jump, {o_j_e} {:?}",  trk[new_trk.o_j_i]);
    }

    // even jump
    let mut e_j_e  =tail.partition_point(|elt| elt.val <= new_trk.val);
    if e_j_e != 0 {
        let val = tail[e_j_e - 1].val;
        // reverse search for first elt
        let e_j_e = tail[0..e_j_e].partition_point(|elt| elt.val != val);
        new_trk.e_j_i = tail[e_j_e].idx;
        new_trk.e_s = trk[new_trk.e_j_i].o_s;
        //println!("Hit Even jump, {e_j_e} {:?}",  trk[new_trk.e_j_i]);
    } 
    trk[idx] = new_trk;
    tail.insert(o_j_e, Vidx{val: new_trk.val, idx});
    //println!("Processed {new_trk:?} {tail:?}");
    //println!("======");
}

impl Solution {
    pub fn odd_even_jumps(arr: Vec<i32>) -> i32 {        
        let mut trk = 
        (0..arr.len()).map(|idx| Trck {
            idx,
            val: arr[idx],
            o_j_i: usize::MAX,
            e_j_i: usize::MAX,
            o_s: false,
            e_s: false
        }).collect::<Vec<_>>();
        let mut tail = Vec::new();
        // sort the tail end of the array with index as the secondary key
        let mut sum = 0;
        for idx in (0..(arr.len())).rev() {
            process_elt(&mut trk, &mut tail, idx);
            if trk[idx].o_s {
                sum += 1;
            }
        }
        sum
    }
}