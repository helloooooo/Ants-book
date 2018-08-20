mod scaneer;
use scaneer::*;
use std::cmp::max;

struct Nap {
    n : i64,
    wv : Vec<Vec<i64>>,
    w : i64,
}

impl Nap {
    fn rec(&self,i:i64,j:i64) -> i64{
        if i == self.n {
            0
        } else if j < self.wv[i as usize][0] {
            self.rec(i+1,j)    
        } else {
            max(self.rec(i + 1,j),self.rec(i + 1, j - self.wv[i as usize][0]) + self.wv[i as usize][1])
        }
    }
}

fn main(){
    let n : i64 = read();
    let wv = read_vec2(n as u32);
    let w : i64 = read();
    let nap = Nap {
         n,
         wv,
         w,
    };
    println!("{}",nap.rec(0, w) )
}
