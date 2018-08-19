
mod scaneer;
use scaneer::*;
fn main(){
    let mut n : i64 = read();
    let mut ln  = read_vec::<i64>();
    // ln.sort();
    let mut ans = 0;
    while n > 1 {
        let (mut mii1, mut mii2) = (0,1);
        if ln[mii1 as usize] > ln[mii2 as usize] {
            let tmp = mii1;
            mii1 = mii2;
            mii2 = tmp;
        }
        for i in 2..n {
            if ln[i as usize] < ln[mii2 as usize]  {
                mii2 = i;
            }
        }

        let t = ln[mii1 as usize] + ln[mii2 as usize];
        ans += t;
        if mii1 == n-1 {
            let tmp = mii1;
            mii1 = mii2;
            mii2 = tmp;
        }
        ln[mii1 as usize] = t;
        ln[mii2 as usize] = ln[(n-1) as usize];
        n -= 1;
    }
    println!("{}",ans);
}