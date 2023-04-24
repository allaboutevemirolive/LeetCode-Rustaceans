// https://leetcode.com/problems/shortest-path-in-a-grid-with-obstacles-elimination/solutions/2773345/a-rust-solution/
//https://leetcode.com/problems/shortest-path-in-a-grid-with-obstacles-elimination/


// use priority_queue::PriorityQueue;
use std::collections::{BinaryHeap, HashMap};

#[allow(unused)]
const M: i64 = 1_000_000_000 + 7;

#[cfg(feature = "local")]
struct Solution {}

#[allow(unused)]
impl Solution {
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = grid.len() as i32;
        let m = grid[0].len() as i32;
        let k = (n+m).min(k);

        let make = |x: i32, y: i32, p: i32, k: i32| (-( (7*p + 5*(-x - y))-1000*k) , (x, y, p, k));
        let neigh = |x: i32, y: i32| -> _ {
            let mut ret = Vec::new();
            if (x > 0) {
                ret.push((x - 1, y + 0));
            }
            if (x < m - 1) {
                ret.push((x + 1, y + 0));
            }
            if (y > 0) {
                ret.push((x + 0, y - 1));
            }
            if (y < n - 1) {
                ret.push((x + 0, y + 1));
            }
            ret
        };

        let mut queue = BinaryHeap::new();
        let mut cost:HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
        cost.insert((0, 0), vec![(0, k)]);

        let mut add = |(_, (x, y,  p, k))| -> bool {
            let get = cost.get(&(x, y));

            if x==1 && y==2 {
                let a=1;
            }
            // println!("c: {:?}",cost.get(&(0,2)));
            if get.is_none() {
                cost.insert((x, y), vec![(p,k)]);
                return true;
            }
            let mut v = cost.get_mut(&(x, y)).unwrap();
            let mut trash = false;
            v.retain(|(pp, kk)| {
                let pp=*pp;
                let kk=*kk;
                if pp<=p {
                    // existing is shorter(or eq)
                    if kk>=k {
                        trash=true;
                    }
                    true
                } else {
                    // new is shorter
                    if kk<=k {
                        false
                    } else {
                        true
                    }
                }
            });
            if !trash {
                v.push((p,k));
            }

            !trash
        };
        queue.push(make(0, 0, 0, k));

        let mut n_extend=0;
        while let Some(e) = queue.pop() {
            let (_, (x, y, p, k)) = e;
            n_extend+=1;
            // println!("extend: {:?}",e);
            if x==0 && y==1 {
                let a=1;
                
            }
            if x==1 && y==2 && k == 1{
                let a=1;
            }

            let n = neigh(x, y);
            for pos in n {
                let c = grid[pos.1 as usize][pos.0 as usize];
                if k - c >= 0 {
                    let new_element = make(pos.0, pos.1, p + 1, k - c);
                    if add(new_element) {
                        queue.push(new_element);
                    }
                }
            }
        }
        if cfg!(feature = "local") {
            println!("n_extend: {:?}",n_extend);
        }
        let o=cost.get(&(m-1,n-1));
        if let Some(v) = o {
            v.iter().map(|(p,_)| *p).min().unwrap_or_else(|| -1)
        } else {
            -1
        }
    }
}
#[cfg(test)]
mod tests {
    use std::{str::FromStr};
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn test1() {
        let f = Solution::shortest_path;
        // let q=f(tx("[[0,0,0],[1,1,0],[0,0,0],[0,1,1],[0,0,0]]"),tx("1"));
        let v = vec![
            vec![0, 0, 0],
            vec![1, 1, 0],
            vec![0, 0, 0],
            vec![0, 1, 1],
            vec![0, 0, 0],
        ];

        // let q=f(tx("[[0,0,0],[1,1,0],[0,0,0],[0,1,1],[0,0,0]]"),tx("1"));
        let res = f(v, tx("1"));

        assert_eq!(res, 6);
    }

    fn tx<T>(s: &str) -> T
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        let q = s.parse();
        q.unwrap()
    }
    #[test]
    fn test2() {
        let f = Solution::shortest_path;
        // let q=f(tx("[[0,0,0],[1,1,0],[0,0,0],[0,1,1],[0,0,0]]"),tx("1"));
        let v = vec![
            vec![0,1,1,1,0,0,0,1,1,0,1,1,1,0,0,1,0,0,1,0,1,0,1,1,0,1,1,0,1,1,1,1,0,0,0,1,1,0,0,1],
            vec![0,0,0,0,0,1,1,0,0,1,1,0,1,0,1,1,1,0,0,0,0,1,1,1,1,1,0,0,0,1,0,0,1,1,1,0,1,1,1,1],
            vec![0,1,1,1,1,1,0,1,0,1,0,1,0,1,0,0,1,0,1,1,0,0,1,1,0,0,1,0,1,0,0,1,0,1,1,0,0,0,1,1],
            vec![0,1,0,1,1,1,1,1,1,0,1,1,0,0,1,1,1,0,1,1,0,1,1,1,1,1,1,1,1,0,0,0,1,0,1,1,0,1,1,1],
            vec![1,0,0,1,0,1,0,1,0,1,0,1,0,0,0,1,1,1,0,0,0,0,1,0,0,1,0,0,1,1,1,0,1,0,1,1,1,0,0,1],
            vec![1,1,1,1,1,1,1,0,0,0,0,0,1,0,1,1,0,0,1,0,1,1,1,1,0,0,1,1,1,0,1,0,1,0,1,1,1,0,1,1],
            vec![0,0,0,1,1,1,0,1,1,1,1,1,0,1,0,0,1,0,1,1,1,1,1,1,0,0,1,1,1,1,1,1,1,1,0,0,1,1,0,1],
            vec![0,1,1,0,1,0,0,1,1,1,0,0,1,0,0,1,0,1,1,1,1,1,1,1,0,1,1,0,1,1,1,0,1,0,0,1,0,0,1,0],
            vec![0,1,0,1,1,0,1,0,0,0,0,1,1,0,0,0,0,1,1,1,1,0,1,0,0,0,0,0,1,1,0,1,1,0,1,1,1,1,1,0],
            vec![0,1,1,1,0,0,0,0,0,0,0,0,0,1,1,1,1,1,0,1,0,0,0,1,1,1,1,1,0,0,0,0,0,0,0,1,1,1,0,0],
            vec![1,0,1,0,0,0,0,0,1,0,0,0,1,1,1,0,1,0,0,0,1,1,0,1,1,1,0,0,1,0,1,1,1,0,0,1,0,0,1,1],
            vec![0,0,1,1,0,0,1,1,1,0,1,1,0,1,0,0,0,1,1,0,0,0,0,1,1,0,1,1,0,0,1,0,0,1,1,0,0,0,1,0],
            vec![0,0,0,0,0,0,0,0,0,1,1,0,0,0,1,0,1,1,0,1,1,0,1,0,0,0,1,0,0,0,1,0,1,1,1,1,1,0,1,1],
            vec![0,1,1,1,1,0,0,0,0,0,1,1,1,1,0,1,1,1,0,0,0,0,1,1,1,0,0,0,1,1,0,1,1,0,1,0,1,0,1,1],
            vec![1,0,0,0,1,1,1,0,0,0,0,1,1,0,1,1,1,1,1,1,0,0,1,1,1,1,0,0,0,0,1,0,0,0,0,0,1,1,0,1],
            vec![1,1,1,0,0,0,1,1,1,0,1,1,1,1,1,1,0,0,1,0,0,0,0,1,1,1,1,1,1,1,0,0,0,1,1,1,0,1,0,1],
            vec![1,0,1,0,1,0,1,1,1,0,1,0,0,0,1,0,0,0,0,1,1,1,0,0,1,1,0,1,1,1,0,1,0,1,0,1,0,1,0,1],
            vec![0,1,1,0,0,0,1,1,0,0,1,0,1,1,1,0,0,1,0,0,1,1,0,1,0,1,0,1,0,0,0,0,1,0,0,0,0,0,0,1],
            vec![1,0,1,0,0,1,0,1,0,1,1,1,1,1,0,1,0,0,0,0,0,0,0,1,0,1,0,1,1,1,1,1,0,0,1,0,0,0,1,1],
            vec![0,1,0,1,0,0,0,1,0,0,1,1,0,0,1,1,0,0,0,1,0,1,1,1,0,1,1,1,0,0,1,1,1,0,0,0,1,0,0,0],
            vec![0,0,1,0,1,1,1,1,0,0,0,0,0,1,0,1,1,0,1,1,1,1,0,0,0,1,0,1,0,1,0,0,0,0,1,0,1,1,0,1],
            vec![0,0,1,1,0,1,1,0,1,0,0,0,1,1,0,0,1,1,1,0,0,0,1,0,0,1,0,1,0,0,0,0,0,1,0,1,1,0,0,0],
            vec![1,1,1,1,0,0,1,0,1,0,1,1,0,1,1,0,0,0,0,1,1,1,1,1,0,1,0,1,0,0,0,0,0,1,0,1,1,1,1,1],
            vec![0,0,0,0,1,1,0,0,0,1,0,1,1,1,1,1,1,0,1,0,0,1,0,1,1,0,0,1,1,0,0,1,1,1,0,1,0,0,1,0]
                    ];

        // let q=f(tx("[[0,0,0],[1,1,0],[0,0,0],[0,1,1],[0,0,0]]"),tx("1"));
        let res = f(v, tx("617"));

        assert_eq!(res, 62);
    }
}