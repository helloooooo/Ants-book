mod scaneer;
use scaneer::*;
use std::cmp::min;

fn main(){
    let v = vec![1,5,10,50,100,500];
    let coin = read_vec::<i64>();
    let mut a = read::<i64>();
    let ans = solve(v,coin,&mut a);
    print!("{}",ans);
}
fn solve(v:Vec<i64>,coin:Vec<i64>,a:&mut i64) -> i64 {
    let ans = (0..6).rev().
    fold(0,|y,x| { 
        let t =  min(*a /v[x as usize],coin[x as usize]);
        *a -= t * v[x as usize];
        y + t
    });
    ans
}