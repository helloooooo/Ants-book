mod scaneer;
use scaneer::*;

fn main(){
    let n:i64 = read();
    let s = read_vec::<i64>();
    let t = read_vec::<i64>();
    let mut p:Vec<(i64,i64)> = s.into_iter().zip(t.into_iter()).collect();
    p.sort_by_key(|&(a,b)| b);
    let ans = p.iter().fold((0,0),|y,x| {
        if y.1 < x.0 {
            (y.0 + 1,x.1)
        }else {
            (y.0,y.1)
        }
    }).0;
    println!("{}",ans );
}