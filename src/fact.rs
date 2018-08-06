mod scaneer;
use scaneer::*;
fn main(){
    let s : i64 = read();
    println!("{}",fact(s) );
}
fn fact(n:i64) -> i64{
    if n == 0{
        1
    } else {
        n * fact(n - 1)
    }
}