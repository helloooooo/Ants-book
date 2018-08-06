mod scaneer;
use scaneer::*;
fn main(){
    let n:i64 = read();
    let a = read_vec::<i64>();
    let k:i64 = read();
    if dfs(0,0,&a,n,k) {
        println!("Yes");
    } else{
        println!("No");
    }
}

fn dfs(i:i64,sum:i64,a:&Vec<i64>,n:i64,k:i64)->bool {
    if i == n { return sum == k; } 
    if dfs(i+1,sum,a,n,k) { return true; }
    if dfs(i+1,sum+a[i as usize],a,n,k) { return true;}
    return false;
}