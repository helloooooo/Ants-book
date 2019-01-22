use scaneer::*;

pub fn solve(){
    let n = read::<i64>();
    let an = read_vec::<i64>();
    let m = read::<i64>();
    let bn = read_vec::<i64>();
    linerSearch(n,an,m,bn);
}

pub fn linerSearch(n:i64,an:Vec<i64>,m:i64,bn:Vec<i64>) -> i64{
    let mut ans = 0;
    for v in &bn {
        if an.contains(&v) {
            ans += 1;
        }
    }
    return ans;
}
pub fn gate_linerSearch(n:i64,mut an:Vec<i64>,m:i64,bn:Vec<i64>) -> i64{
    let mut i = 0;
    let mut ans:i64 = 0;
    for b in bn {
        let mut i = 0;
        an.push(b);
        while an[i] != b{
            i+=1;
        }
        if i as i64 != n {
            ans += 1;
        }
    }
    return ans;
}