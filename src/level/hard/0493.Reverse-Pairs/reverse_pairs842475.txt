// https://leetcode.com/problems/reverse-pairs/solutions/842475/bit-rust-version/
#[derive(Clone,Copy,Debug)]
enum T{
    Single,
    Double
}

impl Solution {
    pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
        let x=nums.iter().enumerate().map(|(i,x)|(i+1,x.clone() as i64,T::Single));
        let y=nums.iter().enumerate().map(|(i,x)|(i+1,(x.clone() as i64)*2,T::Double));
        let g=x.chain(y);
        let mut z:Vec<(usize,i64,T)>=g.collect();
        z.sort_by(|x,y|x.1.cmp(&y.1));
        //println!("{:?}",z);
        let mut  res:i32=0;
        let mut bit=BIT::new(nums.len());
        
        let mut n:i32=0;
        
        for (i,x,t) in z{
            match t{
                T::Single=>{
                    let x=bit.sum( i as i32);
                    res+=(n-x);
                }
                T::Double=>{
                    bit.update(i as i32);
                    n+=1;
                }
            }
        }
        
        res
    }
}

struct BIT{
    n:usize,
    b:[i32;50010],
}

impl BIT{
    fn new(n:usize)->Self{
        BIT{
            n,
            b:[0;50010]
        }
    }
    
    fn update(&mut self,mut i:i32){
        while i as usize<=self.n{
            self.b[i as usize]+=1;
            i+=i&(-i);
        }
    }
    
    fn sum(&self,mut i:i32)->i32{
        let mut res:i32=0;
        while i>0{
            res+=self.b[i as usize];
            i-=i&-i;
        }
        res
    }
}