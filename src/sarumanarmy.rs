mod scaneer;
use scaneer::*;

fn main(){
    let n :i64 = read();
    let r :i64 = read();
    let mut x  = read_vec::<i64>();
    x.sort();
    let mut i = 0;
    let mut ans = 0;
    while i < n{
        let s = x[i as usize];
        i = i + 1;
        while i < n && x[i as usize] <= s + r {
            i = i + 1;
        }
        let p = x[(i - 1) as usize];
        // println!("{}",p);
        while i < n && x[i as usize] <= p + r {
            i = i + 1;
        }
        ans += 1;
    }
    println!("{}",ans);
}