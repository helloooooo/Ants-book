mod scaneer;
use scaneer::*;

fn main(){
    let nm = read_vec::<i64>();
    let (n,m) = (nm[0],nm[1]);
    let mut field :Vec<Vec<char>>= Vec::new();
    for _ in 0..n {
        field.push(read::<String>().chars().collect());
    }
    let ans = solve(n,m,&mut field);
    println!("{}", ans);



}
fn dfs(x:i64,y:i64,field:&mut Vec<Vec<char>>,n:i64,m:i64){
    field[x as usize][y as usize] = '.';
    for j in -1..2 {
        for k in -1..2{
            let (nx,ny) = (x+j,y+k);
            if (0 <= nx && nx < n && 0 <= ny && ny < m && field[nx as usize][ny as usize] == 'W'){
                dfs(nx,ny,field,n,m);
            }
        }
    }
}
fn solve(n:i64,m:i64,field:&mut Vec<Vec<char>>) -> i64{
    let mut ans = 0;
    for j in 0..n {
        for k in 0..m{
            if field[j as usize][k as usize] == 'W'{
                dfs(j,k,field,n,m);
                ans += 1;
            }
        } 
    }
    ans
}