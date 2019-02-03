use scaneer::*;

fn main(){
    let (n,k) = {
        let t = read_vec::<usize>();
        (t[0],t[1])
    };
    let mut v:Vec<usize> = vec![];
    for _ in 0..n {
        v.push(read::<usize>());
    }

}

fn check(p:usize, n:usize,k:usize,v:Vec<usize>) -> usize {
    let mut i = 0;
    for j in 0..k {
        let mut s = 0;
        while s + v[i] <= p {
            s += v[i];
            i += 1;
            if i == n { return n; }
        }
    }
    return i;
}

fn solve() {
    let mut left = 0;
    let mut right = 1e10 as usize;
    while right - left > 1 {
        let mid = (left + right) / 2;
        let res = check(p: usize, n: usize, k: usize, v: Vec<usize>)
    }
}