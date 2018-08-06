mod scaneer;
use scaneer::*;
const MAX_N: usize = 1000;
struct FibMemo{
    memo:[i64;MAX_N],
}
impl FibMemo{
    fn new() -> Self{
        FibMemo { memo:[0;MAX_N]}
    }
    fn fib(&mut self,n:i64) -> i64{
            if n <= 1{ return n; }
            if self.memo[n as usize] != 0 {  return self.memo[n as usize]; }
            self.memo[n as usize] = self.fib(n-1) + self.fib(n-2);
            self.memo[n as usize]
    }
}
fn main(){
    let s : i64 = read();
    let mut f = FibMemo::new();
    println!("{}",f.fib(10) );
}
